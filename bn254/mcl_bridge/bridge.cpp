// Replicates the little-endian ark byte semantics of solana-bn254 3.2.1, a
// mismatch forks consensus.
#include <mcl/bn.h>

#include <cstdint>
#include <cstring>
#include <vector>

namespace {

constexpr int kOk = 0;
// Maps to AltBn128Error::InvalidInputData.
constexpr int kInvalid = -1;
// Maps to AltBn128Error::GroupError.
constexpr int kGroup = -2;

mclBnFp2 g_twist_b;

bool all_zero(const std::uint8_t *p, std::size_t n) {
  for (std::size_t i = 0; i < n; ++i) {
    if (p[i] != 0) {
      return false;
    }
  }
  return true;
}

// Rejects values >= p, the ark canonical range requirement.
bool load_fp(mclBnFp *out, const std::uint8_t *bytes) {
  return mclBnFp_deserialize(out, bytes, 32) == 32;
}

void store_fp(std::uint8_t *out, const mclBnFp *x) {
  mclBnFp_serialize(out, 32, x);
}

// ark SWFlags in the top two bits, the negative plus infinity pair is not
// canonical.
int take_flags(std::uint8_t *top) {
  unsigned flags = *top >> 6;
  if (flags == 3) {
    return -1;
  }
  *top &= 0x3f;
  return static_cast<int>(flags);
}

// The ark negative condition, y sorted after -y as canonical integers.
bool fp_gt(const mclBnFp *a, const mclBnFp *b) {
  std::uint8_t ab[32];
  std::uint8_t bb[32];
  store_fp(ab, a);
  store_fp(bb, b);
  for (int i = 31; i >= 0; --i) {
    if (ab[i] != bb[i]) {
      return ab[i] > bb[i];
    }
  }
  return false;
}

bool fp_is_negative(const mclBnFp *y) {
  mclBnFp neg;
  mclBnFp_neg(&neg, y);
  return fp_gt(y, &neg);
}

// ark orders Fq2 by c1 first, then c0.
bool fp2_gt(const mclBnFp2 *a, const mclBnFp2 *b) {
  if (!mclBnFp_isEqual(&a->d[1], &b->d[1])) {
    return fp_gt(&a->d[1], &b->d[1]);
  }
  return fp_gt(&a->d[0], &b->d[0]);
}

bool fp2_is_negative(const mclBnFp2 *y) {
  mclBnFp2 neg;
  mclBnFp2_neg(&neg, y);
  return fp2_gt(y, &neg);
}

// TryFrom<PodG1> parity, curve check inside the Validate::Yes deserialize.
int load_g1_checked(mclBnG1 *out, const std::uint8_t in[64]) {
  if (all_zero(in, 64)) {
    mclBnG1_clear(out);
    return kOk;
  }
  std::uint8_t buf[64];
  std::memcpy(buf, in, 64);
  int flags = take_flags(&buf[63]);
  if (flags < 0) {
    return kInvalid;
  }
  mclBnG1 p;
  if (!load_fp(&p.x, buf) || !load_fp(&p.y, buf + 32)) {
    return kInvalid;
  }
  if (flags == 1) {
    mclBnG1_clear(out);
    return kOk;
  }
  mclBnFp_setInt(&p.z, 1);
  if (!mclBnG1_isValid(&p)) {
    return kInvalid;
  }
  *out = p;
  return kOk;
}

// subgroup mirrors the TryFrom<PodG2> checks, unchecked mirrors
// into_affine_unchecked.
int load_g2(mclBnG2 *out, const std::uint8_t in[128], bool subgroup) {
  if (all_zero(in, 128)) {
    mclBnG2_clear(out);
    return kOk;
  }
  std::uint8_t buf[128];
  std::memcpy(buf, in, 128);
  int flags = take_flags(&buf[127]);
  if (flags < 0) {
    return kInvalid;
  }
  mclBnG2 p;
  if (!load_fp(&p.x.d[0], buf) || !load_fp(&p.x.d[1], buf + 32) ||
      !load_fp(&p.y.d[0], buf + 64) || !load_fp(&p.y.d[1], buf + 96)) {
    return kInvalid;
  }
  if (flags == 1) {
    mclBnG2_clear(out);
    return kOk;
  }
  mclBnFp_setInt(&p.z.d[0], 1);
  mclBnFp_clear(&p.z.d[1]);
  if (subgroup) {
    if (!mclBnG2_isValid(&p) || !mclBnG2_isValidOrder(&p)) {
      return kInvalid;
    }
  } else if (!mclBnG2_isValid(&p)) {
    return kGroup;
  }
  *out = p;
  return kOk;
}

void store_g1(std::uint8_t out[64], const mclBnG1 *p) {
  if (mclBnG1_isZero(p)) {
    std::memset(out, 0, 64);
    return;
  }
  mclBnG1 affine;
  mclBnG1_normalize(&affine, p);
  store_fp(out, &affine.x);
  store_fp(out + 32, &affine.y);
}

void store_g2(std::uint8_t out[128], const mclBnG2 *p) {
  if (mclBnG2_isZero(p)) {
    std::memset(out, 0, 128);
    return;
  }
  mclBnG2 affine;
  mclBnG2_normalize(&affine, p);
  store_fp(out, &affine.x.d[0]);
  store_fp(out + 32, &affine.x.d[1]);
  store_fp(out + 64, &affine.y.d[0]);
  store_fp(out + 96, &affine.y.d[1]);
}

// Reduction mod r matches ark mul_bigint on the r-torsion points every
// caller validates first.
void load_scalar(mclBnFr *out, const std::uint8_t k[32]) {
  mclBnFr_setLittleEndianMod(out, k, 32);
}

int sqrt_ordered(mclBnFp *out, const mclBnFp *y_squared, bool greatest) {
  mclBnFp root;
  if (mclBnFp_squareRoot(&root, y_squared) != 0) {
    return kInvalid;
  }
  mclBnFp neg;
  mclBnFp_neg(&neg, &root);
  bool root_is_larger = fp_gt(&root, &neg);
  if (greatest == root_is_larger) {
    *out = root;
  } else {
    *out = neg;
  }
  return kOk;
}

int sqrt_ordered_fp2(mclBnFp2 *out, const mclBnFp2 *y_squared, bool greatest) {
  mclBnFp2 root;
  if (mclBnFp2_squareRoot(&root, y_squared) != 0) {
    return kInvalid;
  }
  mclBnFp2 neg;
  mclBnFp2_neg(&neg, &root);
  bool root_is_larger = fp2_gt(&root, &neg);
  if (greatest == root_is_larger) {
    *out = root;
  } else {
    *out = neg;
  }
  return kOk;
}

} // namespace

extern "C" int narsil_mcl_init() {
  int rc = mclBn_init(MCL_BN_SNARK1, MCLBN_COMPILED_TIME_VAR);
  if (rc != 0) {
    return rc;
  }
  // Keep isValid a bare curve check, the order check stays explicit.
  mclBn_verifyOrderG1(0);
  mclBn_verifyOrderG2(0);
  // b of the ark bn254 G2 twist, 3 / (9 + u).
  mclBnFp2 num;
  mclBnFp2 den;
  mclBnFp_setInt(&num.d[0], 3);
  mclBnFp_clear(&num.d[1]);
  mclBnFp_setInt(&den.d[0], 9);
  mclBnFp_setInt(&den.d[1], 1);
  mclBnFp2_inv(&den, &den);
  mclBnFp2_mul(&g_twist_b, &num, &den);
  return 0;
}

extern "C" int narsil_mcl_g1_add(std::uint8_t out[64], const std::uint8_t a[64],
                                 const std::uint8_t b[64]) {
  mclBnG1 p;
  mclBnG1 q;
  int rc = load_g1_checked(&p, a);
  if (rc != kOk) {
    return rc;
  }
  rc = load_g1_checked(&q, b);
  if (rc != kOk) {
    return rc;
  }
  mclBnG1 sum;
  mclBnG1_add(&sum, &p, &q);
  store_g1(out, &sum);
  return kOk;
}

extern "C" int narsil_mcl_g1_mul(std::uint8_t out[64], const std::uint8_t p_in[64],
                                 const std::uint8_t k[32]) {
  mclBnG1 p;
  int rc = load_g1_checked(&p, p_in);
  if (rc != kOk) {
    return rc;
  }
  mclBnFr fr;
  load_scalar(&fr, k);
  mclBnG1 product;
  mclBnG1_mul(&product, &p, &fr);
  store_g1(out, &product);
  return kOk;
}

extern "C" int narsil_mcl_g2_add(std::uint8_t out[128], const std::uint8_t a[128],
                                 const std::uint8_t b[128]) {
  mclBnG2 p;
  mclBnG2 q;
  int rc = load_g2(&p, a, false);
  if (rc != kOk) {
    return rc;
  }
  rc = load_g2(&q, b, false);
  if (rc != kOk) {
    return rc;
  }
  mclBnG2 sum;
  mclBnG2_add(&sum, &p, &q);
  store_g2(out, &sum);
  return kOk;
}

extern "C" int narsil_mcl_g2_mul(std::uint8_t out[128], const std::uint8_t p_in[128],
                                 const std::uint8_t k[32]) {
  mclBnG2 p;
  int rc = load_g2(&p, p_in, true);
  if (rc != kOk) {
    return rc;
  }
  mclBnFr fr;
  load_scalar(&fr, k);
  mclBnG2 product;
  mclBnG2_mul(&product, &p, &fr);
  store_g2(out, &product);
  return kOk;
}

extern "C" int narsil_mcl_pairing_is_one(const std::uint8_t *pairs,
                                         std::size_t count, int *is_one) {
  std::vector<mclBnG1> g1s;
  std::vector<mclBnG2> g2s;
  g1s.reserve(count);
  g2s.reserve(count);
  for (std::size_t i = 0; i < count; ++i) {
    const std::uint8_t *pair = pairs + i * 192;
    mclBnG1 p;
    mclBnG2 q;
    int rc = load_g1_checked(&p, pair);
    if (rc != kOk) {
      return rc;
    }
    rc = load_g2(&q, pair + 64, true);
    if (rc != kOk) {
      return rc;
    }
    // ark multi_pairing drops a pair when either side is zero.
    if (mclBnG1_isZero(&p) || mclBnG2_isZero(&q)) {
      continue;
    }
    g1s.push_back(p);
    g2s.push_back(q);
  }
  if (g1s.empty()) {
    *is_one = 1;
    return kOk;
  }
  mclBnGT miller;
  mclBn_millerLoopVec(&miller, g1s.data(), g2s.data(), g1s.size());
  mclBnGT result;
  mclBn_finalExp(&result, &miller);
  *is_one = mclBnGT_isOne(&result);
  return kOk;
}

extern "C" int narsil_mcl_g1_compress(std::uint8_t out[32],
                                      const std::uint8_t in[64]) {
  std::uint8_t buf[64];
  std::memcpy(buf, in, 64);
  int flags = take_flags(&buf[63]);
  if (flags < 0) {
    return kInvalid;
  }
  mclBnFp x;
  mclBnFp y;
  if (!load_fp(&x, buf) || !load_fp(&y, buf + 32)) {
    return kInvalid;
  }
  if (flags == 1) {
    std::memset(out, 0, 32);
    out[31] = 0x40;
    return kOk;
  }
  store_fp(out, &x);
  if (fp_is_negative(&y)) {
    out[31] |= 0x80;
  }
  return kOk;
}

extern "C" int narsil_mcl_g1_decompress(std::uint8_t out[64],
                                        const std::uint8_t in[32]) {
  std::uint8_t buf[32];
  std::memcpy(buf, in, 32);
  int flags = take_flags(&buf[31]);
  if (flags < 0) {
    return kInvalid;
  }
  mclBnFp x;
  if (!load_fp(&x, buf)) {
    return kInvalid;
  }
  if (flags == 1) {
    std::memset(out, 0, 64);
    return kOk;
  }
  mclBnFp y_squared;
  mclBnFp_sqr(&y_squared, &x);
  mclBnFp_mul(&y_squared, &y_squared, &x);
  mclBnFp b;
  mclBnFp_setInt(&b, 3);
  mclBnFp_add(&y_squared, &y_squared, &b);
  mclBnFp y;
  // Positive flag selects the smaller root, the ark convention.
  int rc = sqrt_ordered(&y, &y_squared, flags == 2);
  if (rc != kOk) {
    return rc;
  }
  store_fp(out, &x);
  store_fp(out + 32, &y);
  return kOk;
}

extern "C" int narsil_mcl_g2_compress(std::uint8_t out[64],
                                      const std::uint8_t in[128]) {
  std::uint8_t buf[128];
  std::memcpy(buf, in, 128);
  int flags = take_flags(&buf[127]);
  if (flags < 0) {
    return kInvalid;
  }
  mclBnFp2 x;
  mclBnFp2 y;
  if (!load_fp(&x.d[0], buf) || !load_fp(&x.d[1], buf + 32) ||
      !load_fp(&y.d[0], buf + 64) || !load_fp(&y.d[1], buf + 96)) {
    return kInvalid;
  }
  if (flags == 1) {
    std::memset(out, 0, 64);
    out[63] = 0x40;
    return kOk;
  }
  store_fp(out, &x.d[0]);
  store_fp(out + 32, &x.d[1]);
  if (fp2_is_negative(&y)) {
    out[63] |= 0x80;
  }
  return kOk;
}

extern "C" int narsil_mcl_g2_decompress(std::uint8_t out[128],
                                        const std::uint8_t in[64]) {
  std::uint8_t buf[64];
  std::memcpy(buf, in, 64);
  int flags = take_flags(&buf[63]);
  if (flags < 0) {
    return kInvalid;
  }
  mclBnFp2 x;
  if (!load_fp(&x.d[0], buf) || !load_fp(&x.d[1], buf + 32)) {
    return kInvalid;
  }
  if (flags == 1) {
    std::memset(out, 0, 128);
    return kOk;
  }
  mclBnFp2 y_squared;
  mclBnFp2_sqr(&y_squared, &x);
  mclBnFp2_mul(&y_squared, &y_squared, &x);
  mclBnFp2_add(&y_squared, &y_squared, &g_twist_b);
  mclBnFp2 y;
  int rc = sqrt_ordered_fp2(&y, &y_squared, flags == 2);
  if (rc != kOk) {
    return rc;
  }
  store_fp(out, &x.d[0]);
  store_fp(out + 32, &x.d[1]);
  store_fp(out + 64, &y.d[0]);
  store_fp(out + 96, &y.d[1]);
  return kOk;
}
