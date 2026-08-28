#[cfg(not(target_os = "solana"))]
use crate::target_arch::{narsil_endianness, narsil_error, Endianness};
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
    let version = match version {
        VersionedG1Multiplication::V0 => helius_narsil::flat::G1MultiplicationVersion::V0,
        VersionedG1Multiplication::V1 => helius_narsil::flat::G1MultiplicationVersion::V1,
    };
    helius_narsil::flat::alt_bn128_g1_multiplication(version, input, narsil_endianness(endianness))
        .map_err(narsil_error)
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
    let version = match _version {
        VersionedG2Multiplication::V0 => helius_narsil::flat::G2MultiplicationVersion::V0,
    };
    helius_narsil::flat::alt_bn128_g2_multiplication(version, input, narsil_endianness(endianness))
        .map_err(narsil_error)
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
