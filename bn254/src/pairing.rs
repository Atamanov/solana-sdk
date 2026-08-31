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
    ark_bn254::{self, Config},
    ark_ec::{bn::Bn, pairing::Pairing},
    ark_ff::One,
    target_arch::{G1, G2},
};
#[cfg(all(not(target_os = "solana"), not(solana_bn254_backend = "narsil")))]
use crate::{consts::ALT_BN128_G1_POINT_SIZE as G1_POINT_SIZE, PodG1, PodG2};
use crate::{
    consts::{ALT_BN128_G1_POINT_SIZE, ALT_BN128_G2_POINT_SIZE},
    AltBn128Error, LE_FLAG,
};
#[cfg(target_os = "solana")]
use solana_define_syscall::definitions as syscalls;

/// Pair element size.
pub const ALT_BN128_PAIRING_ELEMENT_SIZE: usize = ALT_BN128_G1_POINT_SIZE + ALT_BN128_G2_POINT_SIZE; // 192
/// Output size for pairing operation.
pub const ALT_BN128_PAIRING_OUTPUT_SIZE: usize = 32;

#[deprecated(
    since = "3.1.0",
    note = "Please use `ALT_BN128_PAIRING_ELEMENT_SIZE` instead"
)]
pub const ALT_BN128_PAIRING_ELEMENT_LEN: usize = ALT_BN128_PAIRING_ELEMENT_SIZE;
#[deprecated(
    since = "3.1.0",
    note = "Please use `ALT_BN128_PAIRING_OUTPUT_SIZE` instead"
)]
pub const ALT_BN128_PAIRING_OUTPUT_LEN: usize = ALT_BN128_PAIRING_OUTPUT_SIZE;

pub const ALT_BN128_PAIRING_BE: u64 = 3;
#[deprecated(since = "3.1.0", note = "Please use `ALT_BN128_PAIRING_BE` instead")]
pub const ALT_BN128_PAIRING: u64 = ALT_BN128_PAIRING_BE;
pub const ALT_BN128_PAIRING_LE: u64 = ALT_BN128_PAIRING_BE | LE_FLAG;

/// The version enum used to version changes to the `alt_bn128_pairing` syscall.
#[cfg(not(target_os = "solana"))]
pub enum VersionedPairing {
    V0,
    /// SIMD-0334 - Fix alt_bn128_pairing Syscall Length Check
    V1,
}

/// The syscall implementation for the `alt_bn128_pairing` syscall.
///
/// This function is intended to be used by the Agave validator client and exists primarily
/// for validator code. Solana programs or other downstream projects should use
/// `alt_bn128_pairing` or `alt_bn128_pairing_le` instead.
///
/// # Warning
///
/// Developers should be extremely careful when modifying this function, as a breaking change
/// can result in a fork in the Solana cluster. Any such change requires an
/// approved Solana SIMD. Subsequently, a new `VersionedPairing` variant must be added,
/// and the new logic must be scoped to that variant.
#[cfg(not(target_os = "solana"))]
pub fn alt_bn128_versioned_pairing(
    version: VersionedPairing,
    input: &[u8],
    endianness: Endianness,
) -> Result<Vec<u8>, AltBn128Error> {
    #[cfg(solana_bn254_backend = "narsil")]
    {
        let version = match version {
            VersionedPairing::V0 => helius_narsil::flat::PairingVersion::V0,
            VersionedPairing::V1 => helius_narsil::flat::PairingVersion::V1,
        };
        helius_narsil::flat::alt_bn128_pairing(version, input, narsil_endianness(endianness))
            .map_err(narsil_error)
    }
    #[cfg(not(solana_bn254_backend = "narsil"))]
    {
        match version {
            VersionedPairing::V0 => {
                if input
                    .len()
                    .checked_rem(ALT_BN128_PAIRING_ELEMENT_SIZE)
                    .is_none()
                {
                    return Err(AltBn128Error::InvalidInputData);
                }
            }
            VersionedPairing::V1 =>
            {
                #[allow(clippy::manual_is_multiple_of)]
                if input.len() % ALT_BN128_PAIRING_ELEMENT_SIZE != 0 {
                    return Err(AltBn128Error::InvalidInputData);
                }
            }
        }

        let ele_len = input.len().saturating_div(ALT_BN128_PAIRING_ELEMENT_SIZE);

        let mut pairs: Vec<(PodG1, PodG2)> = Vec::with_capacity(ele_len);
        for chunk in input.chunks(ALT_BN128_PAIRING_ELEMENT_SIZE).take(ele_len) {
            let (p_bytes, q_bytes) = chunk.split_at(G1_POINT_SIZE);

            let g1 = match endianness {
                Endianness::BE => PodG1::from_be_bytes(p_bytes)?,
                Endianness::LE => PodG1::from_le_bytes(p_bytes)?,
            };
            let g2 = match endianness {
                Endianness::BE => PodG2::from_be_bytes(q_bytes)?,
                Endianness::LE => PodG2::from_le_bytes(q_bytes)?,
            };

            pairs.push((g1, g2));
        }

        let is_one = pairing_is_one(&pairs)?;

        let mut output = [0u8; ALT_BN128_PAIRING_OUTPUT_SIZE];
        if is_one {
            match endianness {
                Endianness::BE => output[ALT_BN128_PAIRING_OUTPUT_SIZE - 1] = 1,
                Endianness::LE => output[0] = 1,
            }
        }
        Ok(output.to_vec())
    }
}

#[inline(always)]
pub fn alt_bn128_pairing_be(input: &[u8]) -> Result<Vec<u8>, AltBn128Error> {
    #[cfg(not(target_os = "solana"))]
    {
        alt_bn128_versioned_pairing(VersionedPairing::V1, input, Endianness::BE)
    }
    #[cfg(target_os = "solana")]
    {
        if input.len() % ALT_BN128_PAIRING_ELEMENT_SIZE != 0 {
            return Err(AltBn128Error::InvalidInputData);
        }
        // SAFETY: This is sound as sol_alt_bn128_group_op pairing always fills all 32 bytes of our buffer
        let mut result_buffer = Vec::with_capacity(ALT_BN128_PAIRING_OUTPUT_SIZE);
        unsafe {
            let result = syscalls::sol_alt_bn128_group_op(
                ALT_BN128_PAIRING_BE,
                input as *const _ as *const u8,
                input.len() as u64,
                result_buffer.as_mut_ptr(),
            );
            match result {
                0 => {
                    result_buffer.set_len(ALT_BN128_PAIRING_OUTPUT_SIZE);
                    Ok(result_buffer)
                }
                _ => Err(AltBn128Error::UnexpectedError),
            }
        }
    }
}

#[deprecated(since = "3.1.0", note = "Please use `alt_bn128_pairing_be` instead")]
#[allow(deprecated)]
#[inline(always)]
pub fn alt_bn128_pairing(input: &[u8]) -> Result<Vec<u8>, AltBn128Error> {
    #[cfg(not(target_os = "solana"))]
    {
        alt_bn128_versioned_pairing(VersionedPairing::V0, input, Endianness::BE)
    }
    #[cfg(target_os = "solana")]
    {
        let mut result_buffer = [0u8; 32];
        let result = unsafe {
            syscalls::sol_alt_bn128_group_op(
                ALT_BN128_PAIRING,
                input as *const _ as *const u8,
                input.len() as u64,
                &mut result_buffer as *mut _ as *mut u8,
            )
        };

        match result {
            0 => Ok(result_buffer.to_vec()),
            _ => Err(AltBn128Error::UnexpectedError),
        }
    }
}

#[inline(always)]
pub fn alt_bn128_pairing_le(input: &[u8]) -> Result<Vec<u8>, AltBn128Error> {
    #[cfg(not(target_os = "solana"))]
    {
        alt_bn128_versioned_pairing(VersionedPairing::V1, input, Endianness::LE)
    }
    #[cfg(target_os = "solana")]
    {
        if input.len() % ALT_BN128_PAIRING_ELEMENT_SIZE != 0 {
            return Err(AltBn128Error::InvalidInputData);
        }
        // SAFETY: This is sound as sol_alt_bn128_group_op pairing always fills all 32 bytes of our buffer
        let mut result_buffer = Vec::with_capacity(ALT_BN128_PAIRING_OUTPUT_SIZE);
        unsafe {
            let result = syscalls::sol_alt_bn128_group_op(
                ALT_BN128_PAIRING_LE,
                input as *const _ as *const u8,
                input.len() as u64,
                result_buffer.as_mut_ptr(),
            );
            match result {
                0 => {
                    result_buffer.set_len(ALT_BN128_PAIRING_OUTPUT_SIZE);
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
fn pairing_is_one(pairs: &[(PodG1, PodG2)]) -> Result<bool, AltBn128Error> {
    let mut vec_pairs: Vec<(G1, G2)> = Vec::with_capacity(pairs.len());
    for (g1, g2) in pairs {
        vec_pairs.push(((*g1).try_into()?, (*g2).try_into()?));
    }

    let res = <Bn<Config> as Pairing>::multi_pairing(
        vec_pairs.iter().map(|pair| pair.0),
        vec_pairs.iter().map(|pair| pair.1),
    );

    Ok(res.0 == ark_bn254::Fq12::one())
}

#[cfg(all(not(target_os = "solana"), solana_bn254_backend = "mcl"))]
fn pairing_is_one(pairs: &[(PodG1, PodG2)]) -> Result<bool, AltBn128Error> {
    let mut pairs_le =
        Vec::with_capacity(pairs.len().saturating_mul(ALT_BN128_PAIRING_ELEMENT_SIZE));
    for (g1, g2) in pairs {
        pairs_le.extend_from_slice(&g1.0);
        pairs_le.extend_from_slice(&g2.0);
    }
    mcl::pairing_is_one(&pairs_le)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alt_bn128_pairing_invalid_length() {
        let input = [0; 193];
        let result = alt_bn128_pairing_be(&input);
        assert!(result.is_err());
    }
}
