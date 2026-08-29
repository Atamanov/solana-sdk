//! Digests pinned from the stock solana-poseidon 5.0.0 light-poseidon backend.

use solana_poseidon::{hashv, Endianness, Parameters};

const VECTOR_SET_SHA256: &str = "9980f6b3ddf2bd0d1165a6409abec9866b3d51d5e42c5fdd1b0688ffc913938b";

fn hashv_bytes(endianness: Endianness, input: &[u8]) -> Vec<u8> {
    let vals: Vec<&[u8]> = input.chunks(32).collect();
    hashv(Parameters::Bn254X5, endianness, &vals)
        .unwrap()
        .to_bytes()
        .to_vec()
}

fn hashv_be(input: &[u8]) -> Vec<u8> {
    hashv_bytes(Endianness::BigEndian, input)
}

fn hashv_le(input: &[u8]) -> Vec<u8> {
    hashv_bytes(Endianness::LittleEndian, input)
}

const VECTORS: &[Vector] = &[
    Vector {
        name: "hashv_be_w1",
        run: hashv_be,
        input: "0000000000000000000000000000000000000000000000000000000000000001",
        expected: "29176100eaa962bdc1fe6c654d6a3c130e96a4d1168b33848b897dc502820133",
    },
    Vector {
        name: "hashv_le_w1",
        run: hashv_le,
        input: "0100000000000000000000000000000000000000000000000000000000000000",
        expected: "33018202c57d898b84338b16d1a4960e133c6a4d656cfec1bd62a9ea00611729",
    },
    Vector {
        name: "hashv_be_w2",
        run: hashv_be,
        input: "01010101010101010101010101010101010101010101010101010101010101010202020202020202020202020202020202020202020202020202020202020202",
        expected: "0d54e1938f8a8c1c7deb5e0355f26319207b84fe9ca2ce1b26e735c829821990",
    },
    Vector {
        name: "hashv_le_w2",
        run: hashv_le,
        input: "01010101010101010101010101010101010101010101010101010101010101010202020202020202020202020202020202020202020202020202020202020202",
        expected: "90198229c835e7261bcea29cfe847b201963f255035eeb7d1c8c8a8f93e1540d",
    },
    Vector {
        name: "hashv_be_w12",
        run: hashv_be,
        input: "000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001",
        expected: "14390be0baef249bd47c65ddac65c2e52e8513c081c1cd72c98006098e9a8fbe",
    },
    Vector {
        name: "hashv_le_w12",
        run: hashv_le,
        input: "010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000",
        expected: "be8f9a8e090680c972cdc181c013852ee5c265acdd657cd49b24efbae00b3914",
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
