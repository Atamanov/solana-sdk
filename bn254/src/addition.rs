#[cfg(not(target_os = "solana"))]
use crate::target_arch::{narsil_endianness, narsil_error, Endianness};
use crate::{
    consts::{ALT_BN128_G1_POINT_SIZE, ALT_BN128_G2_POINT_SIZE},
    AltBn128Error, LE_FLAG,
};
#[cfg(target_os = "solana")]
use solana_define_syscall::definitions as syscalls;

/// Input size for the g1 add operation.
pub const ALT_BN128_G1_ADDITION_INPUT_SIZE: usize = ALT_BN128_G1_POINT_SIZE * 2; // 128

/// Input size for the g2 add operation.
pub const ALT_BN128_G2_ADDITION_INPUT_SIZE: usize = ALT_BN128_G2_POINT_SIZE * 2; // 256

#[deprecated(
    since = "3.2.0",
    note = "Please use `ALT_BN128_G1_ADDITION_INPUT_SIZE` instead"
)]
pub const ALT_BN128_ADDITION_INPUT_SIZE: usize = ALT_BN128_G1_ADDITION_INPUT_SIZE;
#[deprecated(since = "3.2.0", note = "Please use `ALT_BN128_G1_POINT_SIZE` instead")]
pub const ALT_BN128_ADDITION_OUTPUT_SIZE: usize = ALT_BN128_G1_POINT_SIZE;

#[deprecated(
    since = "3.1.0",
    note = "Please use `ALT_BN128_G1_ADDITION_INPUT_SIZE` instead"
)]
pub const ALT_BN128_ADDITION_INPUT_LEN: usize = ALT_BN128_G1_ADDITION_INPUT_SIZE;
#[deprecated(since = "3.1.0", note = "Please use `ALT_BN128_G1_POINT_SIZE` instead")]
pub const ALT_BN128_ADDITION_OUTPUT_LEN: usize = ALT_BN128_G1_POINT_SIZE;

pub const ALT_BN128_G1_ADD_BE: u64 = 0;
pub const ALT_BN128_G1_SUB_BE: u64 = 1;
#[deprecated(since = "3.1.0", note = "Please use `ALT_BN128_G1_ADD_BE` instead")]
pub const ALT_BN128_ADD: u64 = ALT_BN128_G1_ADD_BE;
#[deprecated(since = "3.1.0", note = "Please use `ALT_BN128_G1_SUB_BE` instead")]
pub const ALT_BN128_SUB: u64 = ALT_BN128_G1_SUB_BE;
pub const ALT_BN128_G2_ADD_BE: u64 = 4;
pub const ALT_BN128_G2_SUB_BE: u64 = 5;
pub const ALT_BN128_G1_ADD_LE: u64 = ALT_BN128_G1_ADD_BE | LE_FLAG;
pub const ALT_BN128_G1_SUB_LE: u64 = ALT_BN128_G1_SUB_BE | LE_FLAG;
pub const ALT_BN128_G2_ADD_LE: u64 = ALT_BN128_G2_ADD_BE | LE_FLAG;
pub const ALT_BN128_G2_SUB_LE: u64 = ALT_BN128_G2_SUB_BE | LE_FLAG;

/// The version enum used to version changes to the `alt_bn128_g1_addition` syscall.
#[cfg(not(target_os = "solana"))]
pub enum VersionedG1Addition {
    V0,
}

/// The version enum used to version changes to the `alt_bn128_g2_addition` syscall.
#[cfg(not(target_os = "solana"))]
pub enum VersionedG2Addition {
    V0,
}

/// The syscall implementation for the `alt_bn128_g1_addition` syscall.
///
/// This function is intended to be used by the Agave validator client and exists primarily
/// for validator code. Solana programs or other downstream projects should use
/// `alt_bn128_g1_addition_be` or `alt_bn128_g1_addition_le` instead.
///
/// # Warning
///
/// Developers should be extremely careful when modifying this function, as a breaking change
/// can result in a fork in the Solana cluster. Any such change requires an
/// approved Solana SIMD. Subsequently, a new `VersionedG1Addition` variant must be added,
/// and the new logic must be scoped to that variant.
#[cfg(not(target_os = "solana"))]
pub fn alt_bn128_versioned_g1_addition(
    _version: VersionedG1Addition,
    input: &[u8],
    endianness: Endianness,
) -> Result<Vec<u8>, AltBn128Error> {
    let version = match _version {
        VersionedG1Addition::V0 => helius_narsil::flat::G1AdditionVersion::V0,
    };
    helius_narsil::flat::alt_bn128_g1_addition(version, input, narsil_endianness(endianness))
        .map_err(narsil_error)
}

#[inline(always)]
pub fn alt_bn128_g1_addition_be(input: &[u8]) -> Result<Vec<u8>, AltBn128Error> {
    #[cfg(not(target_os = "solana"))]
    {
        alt_bn128_versioned_g1_addition(VersionedG1Addition::V0, input, Endianness::BE)
    }
    #[cfg(target_os = "solana")]
    {
        if input.len() > ALT_BN128_G1_ADDITION_INPUT_SIZE {
            return Err(AltBn128Error::InvalidInputData);
        }
        // SAFETY: This is sound as sol_alt_bn128_group_op addition always fills all 64 bytes of our buffer
        let mut result_buffer = Vec::with_capacity(ALT_BN128_G1_POINT_SIZE);
        unsafe {
            let result = syscalls::sol_alt_bn128_group_op(
                ALT_BN128_G1_ADD_BE,
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
    note = "Please use `alt_bn128_g1_addition_be` instead"
)]
#[inline(always)]
pub fn alt_bn128_addition(input: &[u8]) -> Result<Vec<u8>, AltBn128Error> {
    alt_bn128_g1_addition_be(input)
}

#[inline(always)]
pub fn alt_bn128_g1_addition_le(
    input: &[u8; ALT_BN128_G1_ADDITION_INPUT_SIZE],
) -> Result<Vec<u8>, AltBn128Error> {
    #[cfg(not(target_os = "solana"))]
    {
        alt_bn128_versioned_g1_addition(VersionedG1Addition::V0, input, Endianness::LE)
    }
    #[cfg(target_os = "solana")]
    {
        // SAFETY: This is sound as sol_alt_bn128_group_op addition always fills all 64 bytes of our buffer
        let mut result_buffer = Vec::with_capacity(ALT_BN128_G1_POINT_SIZE);
        unsafe {
            let result = syscalls::sol_alt_bn128_group_op(
                ALT_BN128_G1_ADD_LE,
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

/// The syscall implementation for the `alt_bn128_g2_addition` syscall.
///
/// This function is intended to be used by the Agave validator client and exists primarily
/// for validator code. Solana programs or other downstream projects should use
/// `alt_bn128_g2_addition_be` or `alt_bn128_g2_addition_le` instead.
///
/// # Security Note: Unlike G1, which has cofactor 1, the group G2 has a high cofactor.
/// This G2 addition function validates only the curve equation; it does not perform
/// a subgroup (coset) check.
///
/// # Warning
///
/// Developers should be extremely careful when modifying this function, as a breaking change
/// can result in a fork in the Solana cluster. Any such change requires an
/// approved Solana SIMD. Subsequently, a new `VersionedG2Addition` variant must be added,
/// and the new logic must be scoped to that variant.
#[cfg(not(target_os = "solana"))]
pub fn alt_bn128_versioned_g2_addition(
    _version: VersionedG2Addition,
    input: &[u8],
    endianness: Endianness,
) -> Result<Vec<u8>, AltBn128Error> {
    let version = match _version {
        VersionedG2Addition::V0 => helius_narsil::flat::G2AdditionVersion::V0,
    };
    helius_narsil::flat::alt_bn128_g2_addition(version, input, narsil_endianness(endianness))
        .map_err(narsil_error)
}

#[inline(always)]
pub fn alt_bn128_g2_addition_be(
    input: &[u8; ALT_BN128_G2_ADDITION_INPUT_SIZE],
) -> Result<Vec<u8>, AltBn128Error> {
    #[cfg(not(target_os = "solana"))]
    {
        alt_bn128_versioned_g2_addition(VersionedG2Addition::V0, input, Endianness::BE)
    }
    #[cfg(target_os = "solana")]
    {
        // SAFETY: This is sound as sol_alt_bn128_group_op addition always fills all 128 bytes of our buffer
        let mut result_buffer = Vec::with_capacity(ALT_BN128_G2_POINT_SIZE);
        unsafe {
            let result = syscalls::sol_alt_bn128_group_op(
                ALT_BN128_G2_ADD_BE,
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
pub fn alt_bn128_g2_addition_le(
    input: &[u8; ALT_BN128_G2_ADDITION_INPUT_SIZE],
) -> Result<Vec<u8>, AltBn128Error> {
    #[cfg(not(target_os = "solana"))]
    {
        alt_bn128_versioned_g2_addition(VersionedG2Addition::V0, input, Endianness::LE)
    }
    #[cfg(target_os = "solana")]
    {
        // SAFETY: This is sound as sol_alt_bn128_group_op addition always fills all 128 bytes of our buffer
        let mut result_buffer = Vec::with_capacity(ALT_BN128_G2_POINT_SIZE);
        unsafe {
            let result = syscalls::sol_alt_bn128_group_op(
                ALT_BN128_G2_ADD_LE,
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
