//! Outputs pinned from the stock solana-bn254 3.2.1 arkworks backend.
#![cfg(not(target_os = "solana"))]

use solana_bn254::{
    compression::prelude::{
        alt_bn128_g1_compress_be, alt_bn128_g1_compress_le, alt_bn128_g1_decompress_be,
        alt_bn128_g1_decompress_le, alt_bn128_g2_compress_be, alt_bn128_g2_compress_le,
        alt_bn128_g2_decompress_be, alt_bn128_g2_decompress_le,
    },
    versioned::{
        alt_bn128_versioned_g1_addition, alt_bn128_versioned_g1_multiplication,
        alt_bn128_versioned_g2_addition, alt_bn128_versioned_g2_multiplication,
        alt_bn128_versioned_pairing, Endianness, VersionedG1Addition, VersionedG1Multiplication,
        VersionedG2Addition, VersionedG2Multiplication, VersionedPairing,
    },
};

const VECTOR_SET_SHA256: &str = "5ba5cd1578e7c2b3eda9fd3088810785c0628c68f441993059cab1217bfab32c";

fn g1_add_be(input: &[u8]) -> Vec<u8> {
    alt_bn128_versioned_g1_addition(VersionedG1Addition::V0, input, Endianness::BE).unwrap()
}

fn g1_add_le(input: &[u8]) -> Vec<u8> {
    alt_bn128_versioned_g1_addition(VersionedG1Addition::V0, input, Endianness::LE).unwrap()
}

fn g2_add_be(input: &[u8]) -> Vec<u8> {
    alt_bn128_versioned_g2_addition(VersionedG2Addition::V0, input, Endianness::BE).unwrap()
}

fn g2_add_le(input: &[u8]) -> Vec<u8> {
    alt_bn128_versioned_g2_addition(VersionedG2Addition::V0, input, Endianness::LE).unwrap()
}

fn g1_mul_v0_be(input: &[u8]) -> Vec<u8> {
    alt_bn128_versioned_g1_multiplication(VersionedG1Multiplication::V0, input, Endianness::BE)
        .unwrap()
}

fn g1_mul_v0_le(input: &[u8]) -> Vec<u8> {
    alt_bn128_versioned_g1_multiplication(VersionedG1Multiplication::V0, input, Endianness::LE)
        .unwrap()
}

fn g1_mul_v1_be(input: &[u8]) -> Vec<u8> {
    alt_bn128_versioned_g1_multiplication(VersionedG1Multiplication::V1, input, Endianness::BE)
        .unwrap()
}

fn g1_mul_v1_le(input: &[u8]) -> Vec<u8> {
    alt_bn128_versioned_g1_multiplication(VersionedG1Multiplication::V1, input, Endianness::LE)
        .unwrap()
}

fn g2_mul_be(input: &[u8]) -> Vec<u8> {
    alt_bn128_versioned_g2_multiplication(VersionedG2Multiplication::V0, input, Endianness::BE)
        .unwrap()
}

fn g2_mul_le(input: &[u8]) -> Vec<u8> {
    alt_bn128_versioned_g2_multiplication(VersionedG2Multiplication::V0, input, Endianness::LE)
        .unwrap()
}

fn pairing_v0_be(input: &[u8]) -> Vec<u8> {
    alt_bn128_versioned_pairing(VersionedPairing::V0, input, Endianness::BE).unwrap()
}

fn pairing_v0_le(input: &[u8]) -> Vec<u8> {
    alt_bn128_versioned_pairing(VersionedPairing::V0, input, Endianness::LE).unwrap()
}

fn pairing_v1_be(input: &[u8]) -> Vec<u8> {
    alt_bn128_versioned_pairing(VersionedPairing::V1, input, Endianness::BE).unwrap()
}

fn pairing_v1_le(input: &[u8]) -> Vec<u8> {
    alt_bn128_versioned_pairing(VersionedPairing::V1, input, Endianness::LE).unwrap()
}

fn g1_compress_be(input: &[u8]) -> Vec<u8> {
    alt_bn128_g1_compress_be(input).unwrap().to_vec()
}

fn g1_compress_le(input: &[u8]) -> Vec<u8> {
    alt_bn128_g1_compress_le(input).unwrap().to_vec()
}

fn g1_decompress_be(input: &[u8]) -> Vec<u8> {
    alt_bn128_g1_decompress_be(input).unwrap().to_vec()
}

fn g1_decompress_le(input: &[u8]) -> Vec<u8> {
    alt_bn128_g1_decompress_le(input).unwrap().to_vec()
}

fn g2_compress_be(input: &[u8]) -> Vec<u8> {
    alt_bn128_g2_compress_be(input).unwrap().to_vec()
}

fn g2_compress_le(input: &[u8]) -> Vec<u8> {
    alt_bn128_g2_compress_le(input).unwrap().to_vec()
}

fn g2_decompress_be(input: &[u8]) -> Vec<u8> {
    alt_bn128_g2_decompress_be(input).unwrap().to_vec()
}

fn g2_decompress_le(input: &[u8]) -> Vec<u8> {
    alt_bn128_g2_decompress_le(input).unwrap().to_vec()
}

const VECTORS: &[Vector] = &[
    Vector {
        name: "g1_add_v0_be",
        run: g1_add_be,
        input: "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd315ed738c0e0a7c92e7845f96b2ae9c0a68a6a449e3538fc7ff3ebf7a5a18a2c4",
        expected: "0769bf9ac56bea3ff40232bcb1b6bd159315d84715b8e679f2d355961915abf02ab799bee0489429554fdb7c8d086475319e63b40b9c5b57cdf1ff3dd9fe2261",
    },
    Vector {
        name: "g1_add_v0_be_chfast1",
        run: g1_add_be,
        input: "18b18acfb4c2c30276db5411368e7185b311dd124691610c5d3b74034e093dc9063c909c4720840cb5134cb9f59fa749755796819658d32efc0d288198f3726607c2b7f58a84bd6145f00c9c2bc0bb1a187f20ff2c92963a88019e7c6a014eed06614e20c147e940f2d70da3f74c9a17df361706a4485c742bd6788478fa17d7",
        expected: "2243525c5efd4b9c3d3c45ac0ca3fe4dd85e830a4ce6b65fa1eeaee202839703301d1d33be6da8e509df21cc35964723180eed7532537db9ae5e7d48f195c915",
    },
    Vector {
        name: "g1_add_v0_le",
        run: g1_add_le,
        input: "01000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000d3cf876dc108c2d3a81c8716a91678d9851518685b04859b021a132ee7440603c4a2185a7abf3effc78f53e349a4a6680a9caeb2965f84e7927c0a0e8c73ed15",
        expected: "f0ab15199655d3f279e6b81547d8159315bdb6b1bc3202f43fea6bc59abf69076122fed93dfff1cd575b9c0bb4639e317564088d7cdb4f55299448e0be99b72a",
    },
    Vector {
        name: "g2_add_v0_be",
        run: g2_add_be,
        input: "198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa203e205db4f19b37b60121b83a7333706db86431c6d835849957ed8c3928ad7927dc7234fd11d3e8c36c59277c3e6f149d5cd3cfa9a62aee49f8130962b4b3b9195e8aa5b7827463722b8c153931579d3505566b4edf48d498e185f0509de15204bb53b8977e5f92a0bc372742c4830944a59b4fe6b1c0466e2a6dad122b5d2e",
        expected: "1014772f57bb9742735191cd5dcfe4ebbc04156b6878a0a7c9824f32ffb66e8506064e784db10e9051e52826e192715e8d7e478cb09a5e0012defa0694fbc7f5021e2335f3354bb7922ffcc2f38d3323dd9453ac49b55441452aeaca147711b2058e1d5681b5b9e0074b0f9c8d2c68a069b920d74521e79765036d57666c5597",
    },
    Vector {
        name: "g2_add_v0_le",
        run: g2_add_le,
        input: "edf692d95cbdde46ddda5ef7d422436779445c5e66006a42761e1f12efde0018c212f3aeb785e49712e7a9353349aaf1255dfb31b7bf60723a480d9293938e19aa7dfa6601cce64c7bd3430c69e7d1e38f40cb8d8071ab4aeb6d8cdba55ec8125b9722d1dcdaac55f38eb37033314bbc95330c69ad999eec75f05f58d0890609b9b3b4620913f849ee2aa6a9cfd35c9d146f3e7c27596cc3e8d311fd3472dc2779ad28398ced57998435d8c63164b86d7033733ab82101b6379bf1b45d203e202e5d2b12ad6d2a6e46c0b1e64f9ba5440983c4422737bca0925f7e97b853bb0452e19d50f085e198d448df4e6b5605359d573139158c2b72637482b7a58a5e19",
        expected: "f5c7fb9406fade12005e9ab08c477e8d5e7192e12628e551900eb14d784e0606856eb6ff324f82c9a7a078686b1504bcebe4cf5dcd9151734297bb572f77141097556c66576d036597e72145d720b969a0682c8d9c0f4b07e0b9b581561d8e05b2117714caea2a454154b549ac5394dd23338df3c2fc2f92b74b35f335231e02",
    },
    Vector {
        name: "g1_mul_v0_be",
        run: g1_mul_v0_be,
        input: "0000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000070000000000000000000000000000000000000000000000000000000000000000",
        expected: "17072b2ed3bb8d759a5325f477629386cb6fc6ecb801bd76983a6b86abffe078168ada6cd130dd52017bb54bfa19377aadfe3bf05d18f41b77809f7f60d4af9e",
    },
    Vector {
        name: "g1_mul_v0_le",
        run: g1_mul_v0_le,
        input: "0100000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000007000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        expected: "78e0ffab866b3a9876bd01b8ecc66fcb86936277f425539a758dbbd32e2b07179eafd4607f9f80771bf4185df03bfead7a3719fa4bb57b0152dd30d16cda8a16",
    },
    Vector {
        name: "g1_mul_v1_be",
        run: g1_mul_v1_be,
        input: "000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000007",
        expected: "17072b2ed3bb8d759a5325f477629386cb6fc6ecb801bd76983a6b86abffe078168ada6cd130dd52017bb54bfa19377aadfe3bf05d18f41b77809f7f60d4af9e",
    },
    Vector {
        name: "g1_mul_v1_le",
        run: g1_mul_v1_le,
        input: "010000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000700000000000000000000000000000000000000000000000000000000000000",
        expected: "78e0ffab866b3a9876bd01b8ecc66fcb86936277f425539a758dbbd32e2b07179eafd4607f9f80771bf4185df03bfead7a3719fa4bb57b0152dd30d16cda8a16",
    },
    Vector {
        name: "g2_mul_v0_be",
        run: g2_mul_be,
        input: "198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa0000000000000000000000000000000000000000000000000000000000000005",
        expected: "0a09ccf561b55fd99d1c1208dee1162457b57ac5af3759d50671e510e428b2a12e539c423b302d13f4e5773c603948eaf5db5df8ae8a9a9113708390a06410d819b763513924a736e4eebd0d78c91c1bc1d657fee4214057d21414011cfcc7632f8d9f9ab83727c77a2fec063cb7b6e5eb23044ccf535ad49d46d394fb6f6bf6",
    },
    Vector {
        name: "g2_mul_v0_le",
        run: g2_mul_le,
        input: "edf692d95cbdde46ddda5ef7d422436779445c5e66006a42761e1f12efde0018c212f3aeb785e49712e7a9353349aaf1255dfb31b7bf60723a480d9293938e19aa7dfa6601cce64c7bd3430c69e7d1e38f40cb8d8071ab4aeb6d8cdba55ec8125b9722d1dcdaac55f38eb37033314bbc95330c69ad999eec75f05f58d08906090500000000000000000000000000000000000000000000000000000000000000",
        expected: "d81064a090837013919a8aaef85ddbf5ea4839603c77e5f4132d303b429c532ea1b228e410e57106d55937afc57ab5572416e1de08121c9dd95fb561f5cc090af66b6ffb94d3469dd45a53cf4c0423ebe5b6b73c06ec2f7ac72737b89a9f8d2f63c7fc1c011414d2574021e4fe57d6c11b1cc9780dbdeee436a724395163b719",
    },
    Vector {
        name: "pairing_v0_be_1pair",
        run: pairing_v0_be,
        input: "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
        expected: "0000000000000000000000000000000000000000000000000000000000000000",
    },
    Vector {
        name: "pairing_v0_le_1pair",
        run: pairing_v0_le,
        input: "01000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000edf692d95cbdde46ddda5ef7d422436779445c5e66006a42761e1f12efde0018c212f3aeb785e49712e7a9353349aaf1255dfb31b7bf60723a480d9293938e19aa7dfa6601cce64c7bd3430c69e7d1e38f40cb8d8071ab4aeb6d8cdba55ec8125b9722d1dcdaac55f38eb37033314bbc95330c69ad999eec75f05f58d0890609",
        expected: "0000000000000000000000000000000000000000000000000000000000000000",
    },
    Vector {
        name: "pairing_v1_be_1pair",
        run: pairing_v1_be,
        input: "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
        expected: "0000000000000000000000000000000000000000000000000000000000000000",
    },
    Vector {
        name: "pairing_v1_le_1pair",
        run: pairing_v1_le,
        input: "01000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000edf692d95cbdde46ddda5ef7d422436779445c5e66006a42761e1f12efde0018c212f3aeb785e49712e7a9353349aaf1255dfb31b7bf60723a480d9293938e19aa7dfa6601cce64c7bd3430c69e7d1e38f40cb8d8071ab4aeb6d8cdba55ec8125b9722d1dcdaac55f38eb37033314bbc95330c69ad999eec75f05f58d0890609",
        expected: "0000000000000000000000000000000000000000000000000000000000000000",
    },
    Vector {
        name: "pairing_v1_be_2pair",
        run: pairing_v1_be,
        input: "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa000000000000000000000000000000000000000000000000000000000000000130644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd45198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
        expected: "0000000000000000000000000000000000000000000000000000000000000001",
    },
    Vector {
        name: "pairing_v1_le_2pair",
        run: pairing_v1_le,
        input: "01000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000edf692d95cbdde46ddda5ef7d422436779445c5e66006a42761e1f12efde0018c212f3aeb785e49712e7a9353349aaf1255dfb31b7bf60723a480d9293938e19aa7dfa6601cce64c7bd3430c69e7d1e38f40cb8d8071ab4aeb6d8cdba55ec8125b9722d1dcdaac55f38eb37033314bbc95330c69ad999eec75f05f58d0890609010000000000000000000000000000000000000000000000000000000000000045fd7cd8168c203c8dca7168916a81975d588181b64550b829a031e1724e6430edf692d95cbdde46ddda5ef7d422436779445c5e66006a42761e1f12efde0018c212f3aeb785e49712e7a9353349aaf1255dfb31b7bf60723a480d9293938e19aa7dfa6601cce64c7bd3430c69e7d1e38f40cb8d8071ab4aeb6d8cdba55ec8125b9722d1dcdaac55f38eb37033314bbc95330c69ad999eec75f05f58d0890609",
        expected: "0100000000000000000000000000000000000000000000000000000000000000",
    },
    Vector {
        name: "g1_compress_be",
        run: g1_compress_be,
        input: "030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd315ed738c0e0a7c92e7845f96b2ae9c0a68a6a449e3538fc7ff3ebf7a5a18a2c4",
        expected: "030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd3",
    },
    Vector {
        name: "g1_compress_le",
        run: g1_compress_le,
        input: "d3cf876dc108c2d3a81c8716a91678d9851518685b04859b021a132ee7440603c4a2185a7abf3effc78f53e349a4a6680a9caeb2965f84e7927c0a0e8c73ed15",
        expected: "d3cf876dc108c2d3a81c8716a91678d9851518685b04859b021a132ee7440603",
    },
    Vector {
        name: "g1_decompress_be",
        run: g1_decompress_be,
        input: "030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd3",
        expected: "030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd315ed738c0e0a7c92e7845f96b2ae9c0a68a6a449e3538fc7ff3ebf7a5a18a2c4",
    },
    Vector {
        name: "g1_decompress_le",
        run: g1_decompress_le,
        input: "d3cf876dc108c2d3a81c8716a91678d9851518685b04859b021a132ee7440603",
        expected: "d3cf876dc108c2d3a81c8716a91678d9851518685b04859b021a132ee7440603c4a2185a7abf3effc78f53e349a4a6680a9caeb2965f84e7927c0a0e8c73ed15",
    },
    Vector {
        name: "g2_compress_be",
        run: g2_compress_be,
        input: "203e205db4f19b37b60121b83a7333706db86431c6d835849957ed8c3928ad7927dc7234fd11d3e8c36c59277c3e6f149d5cd3cfa9a62aee49f8130962b4b3b9195e8aa5b7827463722b8c153931579d3505566b4edf48d498e185f0509de15204bb53b8977e5f92a0bc372742c4830944a59b4fe6b1c0466e2a6dad122b5d2e",
        expected: "a03e205db4f19b37b60121b83a7333706db86431c6d835849957ed8c3928ad7927dc7234fd11d3e8c36c59277c3e6f149d5cd3cfa9a62aee49f8130962b4b3b9",
    },
    Vector {
        name: "g2_compress_le",
        run: g2_compress_le,
        input: "b9b3b4620913f849ee2aa6a9cfd35c9d146f3e7c27596cc3e8d311fd3472dc2779ad28398ced57998435d8c63164b86d7033733ab82101b6379bf1b45d203e202e5d2b12ad6d2a6e46c0b1e64f9ba5440983c4422737bca0925f7e97b853bb0452e19d50f085e198d448df4e6b5605359d573139158c2b72637482b7a58a5e19",
        expected: "b9b3b4620913f849ee2aa6a9cfd35c9d146f3e7c27596cc3e8d311fd3472dc2779ad28398ced57998435d8c63164b86d7033733ab82101b6379bf1b45d203ea0",
    },
    Vector {
        name: "g2_decompress_be",
        run: g2_decompress_be,
        input: "a03e205db4f19b37b60121b83a7333706db86431c6d835849957ed8c3928ad7927dc7234fd11d3e8c36c59277c3e6f149d5cd3cfa9a62aee49f8130962b4b3b9",
        expected: "203e205db4f19b37b60121b83a7333706db86431c6d835849957ed8c3928ad7927dc7234fd11d3e8c36c59277c3e6f149d5cd3cfa9a62aee49f8130962b4b3b9195e8aa5b7827463722b8c153931579d3505566b4edf48d498e185f0509de15204bb53b8977e5f92a0bc372742c4830944a59b4fe6b1c0466e2a6dad122b5d2e",
    },
    Vector {
        name: "g2_decompress_le",
        run: g2_decompress_le,
        input: "b9b3b4620913f849ee2aa6a9cfd35c9d146f3e7c27596cc3e8d311fd3472dc2779ad28398ced57998435d8c63164b86d7033733ab82101b6379bf1b45d203ea0",
        expected: "b9b3b4620913f849ee2aa6a9cfd35c9d146f3e7c27596cc3e8d311fd3472dc2779ad28398ced57998435d8c63164b86d7033733ab82101b6379bf1b45d203e202e5d2b12ad6d2a6e46c0b1e64f9ba5440983c4422737bca0925f7e97b853bb0452e19d50f085e198d448df4e6b5605359d573139158c2b72637482b7a58a5e19",
    },
];

struct Vector {
    name: &'static str,
    run: fn(&[u8]) -> Vec<u8>,
    input: &'static str,
    expected: &'static str,
}

fn unhex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn sha256(data: &[u8]) -> [u8; 32] {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut state: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let mut message = data.to_vec();
    message.push(0x80);
    while message.len() % 64 != 56 {
        message.push(0);
    }
    message.extend_from_slice(&(data.len() as u64 * 8).to_be_bytes());
    for block in message.chunks_exact(64) {
        let mut w = [0u32; 64];
        for (word, chunk) in w.iter_mut().zip(block.chunks_exact(4)) {
            *word = u32::from_be_bytes(chunk.try_into().unwrap());
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = state;
        for (round_k, round_w) in K.iter().zip(w) {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let t1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(*round_k)
                .wrapping_add(round_w);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        for (word, updated) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *word = word.wrapping_add(updated);
        }
    }
    let mut out = [0u8; 32];
    for (chunk, word) in out.chunks_exact_mut(4).zip(state) {
        chunk.copy_from_slice(&word.to_be_bytes());
    }
    out
}

#[test]
fn outputs_match_pinned_bytes() {
    for vector in VECTORS {
        let output = (vector.run)(&unhex(vector.input));
        assert_eq!(hex(&output), vector.expected, "{}", vector.name);
    }
}

#[test]
fn vector_set_digest_is_pinned() {
    let mut set = Vec::new();
    for vector in VECTORS {
        set.extend(unhex(vector.input));
        set.extend(unhex(vector.expected));
    }
    assert_eq!(hex(&sha256(&set)), VECTOR_SET_SHA256);
}
