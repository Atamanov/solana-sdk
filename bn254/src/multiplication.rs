#[cfg(all(not(target_os = "solana"), solana_bn254_backend = "mcl"))]
use crate::mcl;
#[cfg(not(target_os = "solana"))]
use crate::target_arch::Endianness;
#[cfg(all(not(target_os = "solana"), solana_bn254_backend = "narsil"))]
use crate::target_arch::{narsil_endianness, narsil_error};
#[cfg(all(
    not(target_os = "solana"),
    any(solana_bn254_backend = "ark05", solana_bn254_backend = "ark06")
))]
use crate::{
    ark_ec::AffineRepr,
    ark_ff::BigInteger256,
    ark_serialize::CanonicalDeserialize,
    target_arch::{g1_to_le_bytes, g2_to_le_bytes, G1, G2},
};
#[cfg(all(not(target_os = "solana"), not(solana_bn254_backend = "narsil")))]
use crate::{consts::ALT_BN128_FQ2_SIZE, target_arch::convert_endianness, PodG1, PodG2};
use crate::{
    consts::{ALT_BN128_FIELD_SIZE, ALT_BN128_G1_POINT_SIZE, ALT_BN128_G2_POINT_SIZE},
    AltBn128Error, LE_FLAG,
};
#[cfg(target_os = "solana")]
use solana_define_syscall::definitions as syscalls;

/// Input size for the g1 multiplication operation.
pub const ALT_BN128_G1_MULTIPLICATION_INPUT_SIZE: usize =
    ALT_BN128_G1_POINT_SIZE + ALT_BN128_FIELD_SIZE; // 96

/// Input size for the g2 multiplication operation.
pub const ALT_BN128_G2_MULTIPLICATION_INPUT_SIZE: usize =
    ALT_BN128_G2_POINT_SIZE + ALT_BN128_FIELD_SIZE; // 160

#[deprecated(
    since = "3.2.0",
    note = "Please use `ALT_BN128_G1_MULTIPLICATION_INPUT_SIZE` instead"
)]
pub const ALT_BN128_MULTIPLICATION_INPUT_SIZE: usize = ALT_BN128_G1_MULTIPLICATION_INPUT_SIZE;
#[deprecated(since = "3.2.0", note = "Please use `ALT_BN128_G1_POINT_SIZE` instead")]
pub const ALT_BN128_MULTIPLICATION_OUTPUT_SIZE: usize = ALT_BN128_G1_POINT_SIZE;

#[deprecated(
    since = "3.1.0",
    note = "Please use `ALT_BN128_G1_MULTIPLICATION_INPUT_SIZE` instead"
)]
pub const ALT_BN128_MULTIPLICATION_INPUT_LEN: usize = ALT_BN128_G1_MULTIPLICATION_INPUT_SIZE;
#[deprecated(since = "3.1.0", note = "Please use `ALT_BN128_G1_POINT_SIZE` instead")]
pub const ALT_BN128_MULTIPLICATION_OUTPUT_LEN: usize = ALT_BN128_G1_POINT_SIZE;

pub const ALT_BN128_G1_MUL_BE: u64 = 2;
#[deprecated(since = "3.1.0", note = "Please use `ALT_BN128_G1_MUL_BE` instead")]
pub const ALT_BN128_MUL: u64 = ALT_BN128_G1_MUL_BE;
pub const ALT_BN128_G2_MUL_BE: u64 = 6;
pub const ALT_BN128_G1_MUL_LE: u64 = ALT_BN128_G1_MUL_BE | LE_FLAG;
pub const ALT_BN128_G2_MUL_LE: u64 = ALT_BN128_G2_MUL_BE | LE_FLAG;

/// The version enum used to version changes to the `alt_bn128_g1_multiplication` syscall.
#[cfg(not(target_os = "solana"))]
pub enum VersionedG1Multiplication {
    V0,
    /// SIMD-0222 - Fix alt-bn128-multiplication Syscall Length Check
    V1,
}

/// The version enum used to version changes to the `alt_bn128_g2_multiplication` syscall.
#[cfg(not(target_os = "solana"))]
pub enum VersionedG2Multiplication {
    V0,
}

/// The syscall implementation for the `alt_bn128_g1_multiplication` syscall.
///
/// This function is intended to be used by the Agave validator client and exists primarily
/// for validator code. Solana programs or other downstream projects should use
/// `alt_bn128_g1_multiplication_be` or `alt_bn128_g1_multiplication_le` instead.
///
/// # Warning
///
/// Developers should be extremely careful when modifying this function, as a breaking change
/// can result in a fork in the Solana cluster. Any such change requires an
/// approved Solana SIMD. Subsequently, a new `VersionedG1Multiplication` variant must be added,
/// and the new logic must be scoped to that variant.
#[cfg(not(target_os = "solana"))]
pub fn alt_bn128_versioned_g1_multiplication(
    version: VersionedG1Multiplication,
    input: &[u8],
    endianness: Endianness,
) -> Result<Vec<u8>, AltBn128Error> {
    #[cfg(solana_bn254_backend = "narsil")]
    {
        let version = match version {
            VersionedG1Multiplication::V0 => helius_narsil::flat::G1MultiplicationVersion::V0,
            VersionedG1Multiplication::V1 => helius_narsil::flat::G1MultiplicationVersion::V1,
        };
        helius_narsil::flat::alt_bn128_g1_multiplication(
            version,
            input,
            narsil_endianness(endianness),
        )
        .map_err(narsil_error)
    }
    #[cfg(not(solana_bn254_backend = "narsil"))]
    {
        let expected_length = match version {
            VersionedG1Multiplication::V0 => 128,
            VersionedG1Multiplication::V1 => ALT_BN128_G1_MULTIPLICATION_INPUT_SIZE,
        };

        match endianness {
            Endianness::BE => {
                if input.len() > expected_length {
                    return Err(AltBn128Error::InvalidInputData);
                }
            }
            Endianness::LE => {
                if input.len() != expected_length {
                    return Err(AltBn128Error::InvalidInputData);
                }
            }
        }

        let mut input = input.to_vec();
        match endianness {
            Endianness::BE => input.resize(expected_length, 0),
            Endianness::LE => (),
        }

        let p = match endianness {
            Endianness::BE => PodG1::from_be_bytes(&input[..ALT_BN128_G1_POINT_SIZE])?,
            Endianness::LE => PodG1::from_le_bytes(&input[..ALT_BN128_G1_POINT_SIZE])?,
        };
        let mut fr_bytes = [0u8; ALT_BN128_FIELD_SIZE];
        match endianness {
            Endianness::BE => {
                fr_bytes = convert_endianness::<ALT_BN128_FIELD_SIZE, ALT_BN128_FIELD_SIZE>(
                    &input[ALT_BN128_G1_POINT_SIZE..ALT_BN128_G1_MULTIPLICATION_INPUT_SIZE]
                        .try_into()
                        .unwrap(),
                )
            }
            Endianness::LE => {
                fr_bytes.copy_from_slice(
                    &input[ALT_BN128_G1_POINT_SIZE..ALT_BN128_G1_MULTIPLICATION_INPUT_SIZE],
                );
            }
        }

        let result_point_data = g1_mul(p, &fr_bytes)?;

        match endianness {
            Endianness::BE => Ok(
                convert_endianness::<ALT_BN128_FIELD_SIZE, ALT_BN128_G1_POINT_SIZE>(
                    &result_point_data,
                )
                .to_vec(),
            ),
            Endianness::LE => Ok(result_point_data.to_vec()),
        }
    }
}

#[inline(always)]
pub fn alt_bn128_g1_multiplication_be(input: &[u8]) -> Result<Vec<u8>, AltBn128Error> {
    #[cfg(not(target_os = "solana"))]
    {
        alt_bn128_versioned_g1_multiplication(VersionedG1Multiplication::V1, input, Endianness::BE)
    }
    #[cfg(target_os = "solana")]
    {
        if input.len() > ALT_BN128_G1_MULTIPLICATION_INPUT_SIZE {
            return Err(AltBn128Error::InvalidInputData);
        }
        // SAFETY: This is sound as sol_alt_bn128_group_op multiplication always fills all 64 bytes of our buffer
        let mut result_buffer = Vec::with_capacity(ALT_BN128_G1_POINT_SIZE);
        unsafe {
            let result = syscalls::sol_alt_bn128_group_op(
                ALT_BN128_G1_MUL_BE,
                input as *const _ as *const u8,
                input.len() as u64,
                result_buffer.as_mut_ptr(),
            );
            match result {
                0 => {
                    result_buffer.set_len(ALT_BN128_G1_POINT_SIZE);
                    Ok(result_buffer)
                }
                _ => Err(AltBn128Error::UnexpectedError),
            }
        }
    }
}

#[deprecated(
    since = "3.1.0",
    note = "Please use `alt_bn128_g1_multiplication_be` instead"
)]
#[inline(always)]
pub fn alt_bn128_multiplication(input: &[u8]) -> Result<Vec<u8>, AltBn128Error> {
    alt_bn128_g1_multiplication_be(input)
}

#[inline(always)]
pub fn alt_bn128_g1_multiplication_le(
    input: &[u8; ALT_BN128_G1_MULTIPLICATION_INPUT_SIZE],
) -> Result<Vec<u8>, AltBn128Error> {
    #[cfg(not(target_os = "solana"))]
    {
        alt_bn128_versioned_g1_multiplication(VersionedG1Multiplication::V1, input, Endianness::LE)
    }
    #[cfg(target_os = "solana")]
    {
        // SAFETY: This is sound as sol_alt_bn128_group_op multiplication always fills all 64 bytes of our buffer
        let mut result_buffer = Vec::with_capacity(ALT_BN128_G1_POINT_SIZE);
        unsafe {
            let result = syscalls::sol_alt_bn128_group_op(
                ALT_BN128_G1_MUL_LE,
                input as *const _ as *const u8,
                input.len() as u64,
                result_buffer.as_mut_ptr(),
            );
            match result {
                0 => {
                    result_buffer.set_len(ALT_BN128_G1_POINT_SIZE);
                    Ok(result_buffer)
                }
                _ => Err(AltBn128Error::UnexpectedError),
            }
        }
    }
}

#[deprecated(
    since = "3.1.0",
    note = "Please use `alt_bn128_g1_multiplication_be` instead"
)]
#[cfg(not(target_os = "solana"))]
#[inline(always)]
pub fn alt_bn128_multiplication_128(input: &[u8]) -> Result<Vec<u8>, AltBn128Error> {
    alt_bn128_versioned_g1_multiplication(VersionedG1Multiplication::V0, input, Endianness::BE)
}

/// The syscall implementation for the `alt_bn128_g2_multiplication` syscall.
///
/// This function is intended to be used by the Agave validator client and exists primarily
/// for validator code. Solana programs or other downstream projects should use
/// `alt_bn128_g2_multiplication_be` or `alt_bn128_g2_multiplication_le` instead.
///
/// # Warning
///
/// Developers should be extremely careful when modifying this function, as a breaking change
/// can result in a fork in the Solana cluster. Any such change requires an
/// approved Solana SIMD. Subsequently, a new `VersionedG2Multiplication` variant must be added,
/// and the new logic must be scoped to that variant.
#[cfg(not(target_os = "solana"))]
pub fn alt_bn128_versioned_g2_multiplication(
    _version: VersionedG2Multiplication,
    input: &[u8],
    endianness: Endianness,
) -> Result<Vec<u8>, AltBn128Error> {
    #[cfg(solana_bn254_backend = "narsil")]
    {
        let version = match _version {
            VersionedG2Multiplication::V0 => helius_narsil::flat::G2MultiplicationVersion::V0,
        };
        helius_narsil::flat::alt_bn128_g2_multiplication(
            version,
            input,
            narsil_endianness(endianness),
        )
        .map_err(narsil_error)
    }
    #[cfg(not(solana_bn254_backend = "narsil"))]
    {
        if input.len() != ALT_BN128_G2_MULTIPLICATION_INPUT_SIZE {
            return Err(AltBn128Error::InvalidInputData);
        }

        let p = match endianness {
            Endianness::BE => PodG2::from_be_bytes(&input[..ALT_BN128_G2_POINT_SIZE])?,
            Endianness::LE => PodG2::from_le_bytes(&input[..ALT_BN128_G2_POINT_SIZE])?,
        };
        let mut fr_bytes = [0u8; ALT_BN128_FIELD_SIZE];
        match endianness {
            Endianness::BE => {
                fr_bytes = convert_endianness::<ALT_BN128_FIELD_SIZE, ALT_BN128_FIELD_SIZE>(
                    &input[ALT_BN128_G2_POINT_SIZE..ALT_BN128_G2_MULTIPLICATION_INPUT_SIZE]
                        .try_into()
                        .unwrap(),
                )
            }
            Endianness::LE => {
                fr_bytes.copy_from_slice(
                    &input[ALT_BN128_G2_POINT_SIZE..ALT_BN128_G2_MULTIPLICATION_INPUT_SIZE],
                );
            }
        }

        let result_point_data = g2_mul(p, &fr_bytes)?;

        match endianness {
            Endianness::BE => Ok(
                convert_endianness::<ALT_BN128_FQ2_SIZE, ALT_BN128_G2_POINT_SIZE>(
                    &result_point_data,
                )
                .to_vec(),
            ),
            Endianness::LE => Ok(result_point_data.to_vec()),
        }
    }
}

#[inline(always)]
pub fn alt_bn128_g2_multiplication_be(
    input: &[u8; ALT_BN128_G2_MULTIPLICATION_INPUT_SIZE],
) -> Result<Vec<u8>, AltBn128Error> {
    #[cfg(not(target_os = "solana"))]
    {
        alt_bn128_versioned_g2_multiplication(VersionedG2Multiplication::V0, input, Endianness::BE)
    }
    #[cfg(target_os = "solana")]
    {
        // SAFETY: This is sound as sol_alt_bn128_group_op multiplication always fills all 128 bytes of our buffer
        let mut result_buffer = Vec::with_capacity(ALT_BN128_G2_POINT_SIZE);
        unsafe {
            let result = syscalls::sol_alt_bn128_group_op(
                ALT_BN128_G2_MUL_BE,
                input as *const _ as *const u8,
                input.len() as u64,
                result_buffer.as_mut_ptr(),
            );
            match result {
                0 => {
                    result_buffer.set_len(ALT_BN128_G2_POINT_SIZE);
                    Ok(result_buffer)
                }
                _ => Err(AltBn128Error::UnexpectedError),
            }
        }
    }
}

#[inline(always)]
pub fn alt_bn128_g2_multiplication_le(
    input: &[u8; ALT_BN128_G2_MULTIPLICATION_INPUT_SIZE],
) -> Result<Vec<u8>, AltBn128Error> {
    #[cfg(not(target_os = "solana"))]
    {
        alt_bn128_versioned_g2_multiplication(VersionedG2Multiplication::V0, input, Endianness::LE)
    }
    #[cfg(target_os = "solana")]
    {
        // SAFETY: This is sound as sol_alt_bn128_group_op multiplication always fills all 128 bytes of our buffer
        let mut result_buffer = Vec::with_capacity(ALT_BN128_G2_POINT_SIZE);
        unsafe {
            let result = syscalls::sol_alt_bn128_group_op(
                ALT_BN128_G2_MUL_LE,
                input as *const _ as *const u8,
                input.len() as u64,
                result_buffer.as_mut_ptr(),
            );
            match result {
                0 => {
                    result_buffer.set_len(ALT_BN128_G2_POINT_SIZE);
                    Ok(result_buffer)
                }
                _ => Err(AltBn128Error::UnexpectedError),
            }
        }
    }
}

#[cfg(all(
    not(target_os = "solana"),
    any(solana_bn254_backend = "ark05", solana_bn254_backend = "ark06")
))]
fn g1_mul(
    p: PodG1,
    fr_bytes: &[u8; ALT_BN128_FIELD_SIZE],
) -> Result<[u8; ALT_BN128_G1_POINT_SIZE], AltBn128Error> {
    let p: G1 = p.try_into()?;
    let fr = BigInteger256::deserialize_uncompressed_unchecked(fr_bytes.as_slice())
        .map_err(|_| AltBn128Error::InvalidInputData)?;
    g1_to_le_bytes(p.mul_bigint(fr).into())
}

#[cfg(all(not(target_os = "solana"), solana_bn254_backend = "mcl"))]
fn g1_mul(
    p: PodG1,
    fr_bytes: &[u8; ALT_BN128_FIELD_SIZE],
) -> Result<[u8; ALT_BN128_G1_POINT_SIZE], AltBn128Error> {
    mcl::g1_mul(&p.0, fr_bytes)
}

#[cfg(all(
    not(target_os = "solana"),
    any(solana_bn254_backend = "ark05", solana_bn254_backend = "ark06")
))]
fn g2_mul(
    p: PodG2,
    fr_bytes: &[u8; ALT_BN128_FIELD_SIZE],
) -> Result<[u8; ALT_BN128_G2_POINT_SIZE], AltBn128Error> {
    let p: G2 = p.try_into()?;
    let fr = BigInteger256::deserialize_uncompressed_unchecked(fr_bytes.as_slice())
        .map_err(|_| AltBn128Error::InvalidInputData)?;
    g2_to_le_bytes(p.mul_bigint(fr).into())
}

#[cfg(all(not(target_os = "solana"), solana_bn254_backend = "mcl"))]
fn g2_mul(
    p: PodG2,
    fr_bytes: &[u8; ALT_BN128_FIELD_SIZE],
) -> Result<[u8; ALT_BN128_G2_POINT_SIZE], AltBn128Error> {
    mcl::g2_mul(&p.0, fr_bytes)
}
