//! Buffers are the crate's little-endian Pod layout.

use {crate::AltBn128Error, std::sync::OnceLock};

extern "C" {
    fn narsil_mcl_init() -> i32;
    fn narsil_mcl_g1_add(out: *mut u8, a: *const u8, b: *const u8) -> i32;
    fn narsil_mcl_g1_mul(out: *mut u8, p: *const u8, k: *const u8) -> i32;
    fn narsil_mcl_g2_add(out: *mut u8, a: *const u8, b: *const u8) -> i32;
    fn narsil_mcl_g2_mul(out: *mut u8, p: *const u8, k: *const u8) -> i32;
    fn narsil_mcl_pairing_is_one(pairs: *const u8, count: usize, is_one: *mut i32) -> i32;
    fn narsil_mcl_g1_compress(out: *mut u8, input: *const u8) -> i32;
    fn narsil_mcl_g1_decompress(out: *mut u8, input: *const u8) -> i32;
    fn narsil_mcl_g2_compress(out: *mut u8, input: *const u8) -> i32;
    fn narsil_mcl_g2_decompress(out: *mut u8, input: *const u8) -> i32;
}

fn init_status() -> i32 {
    static STATUS: OnceLock<i32> = OnceLock::new();
    *STATUS.get_or_init(|| unsafe { narsil_mcl_init() })
}

fn map_status(status: i32) -> Result<(), AltBn128Error> {
    match status {
        0 => Ok(()),
        -1 => Err(AltBn128Error::InvalidInputData),
        -2 => Err(AltBn128Error::GroupError),
        _ => Err(AltBn128Error::UnexpectedError),
    }
}

pub(crate) fn g1_add(a: &[u8; 64], b: &[u8; 64]) -> Result<[u8; 64], AltBn128Error> {
    if init_status() != 0 {
        return Err(AltBn128Error::UnexpectedError);
    }
    let mut out = [0u8; 64];
    map_status(unsafe { narsil_mcl_g1_add(out.as_mut_ptr(), a.as_ptr(), b.as_ptr()) })?;
    Ok(out)
}

pub(crate) fn g1_mul(p: &[u8; 64], k: &[u8; 32]) -> Result<[u8; 64], AltBn128Error> {
    if init_status() != 0 {
        return Err(AltBn128Error::UnexpectedError);
    }
    let mut out = [0u8; 64];
    map_status(unsafe { narsil_mcl_g1_mul(out.as_mut_ptr(), p.as_ptr(), k.as_ptr()) })?;
    Ok(out)
}

pub(crate) fn g2_add(a: &[u8; 128], b: &[u8; 128]) -> Result<[u8; 128], AltBn128Error> {
    if init_status() != 0 {
        return Err(AltBn128Error::UnexpectedError);
    }
    let mut out = [0u8; 128];
    map_status(unsafe { narsil_mcl_g2_add(out.as_mut_ptr(), a.as_ptr(), b.as_ptr()) })?;
    Ok(out)
}

pub(crate) fn g2_mul(p: &[u8; 128], k: &[u8; 32]) -> Result<[u8; 128], AltBn128Error> {
    if init_status() != 0 {
        return Err(AltBn128Error::UnexpectedError);
    }
    let mut out = [0u8; 128];
    map_status(unsafe { narsil_mcl_g2_mul(out.as_mut_ptr(), p.as_ptr(), k.as_ptr()) })?;
    Ok(out)
}

pub(crate) fn pairing_is_one(pairs_le: &[u8]) -> Result<bool, AltBn128Error> {
    debug_assert_eq!(pairs_le.len() % 192, 0);
    if init_status() != 0 {
        return Err(AltBn128Error::UnexpectedError);
    }
    let mut is_one = 0i32;
    map_status(unsafe {
        narsil_mcl_pairing_is_one(pairs_le.as_ptr(), pairs_le.len() / 192, &mut is_one)
    })?;
    Ok(is_one != 0)
}

pub(crate) fn g1_compress(input: &[u8; 64]) -> Result<[u8; 32], ()> {
    if init_status() != 0 {
        return Err(());
    }
    let mut out = [0u8; 32];
    match unsafe { narsil_mcl_g1_compress(out.as_mut_ptr(), input.as_ptr()) } {
        0 => Ok(out),
        _ => Err(()),
    }
}

pub(crate) fn g1_decompress(input: &[u8; 32]) -> Result<[u8; 64], ()> {
    if init_status() != 0 {
        return Err(());
    }
    let mut out = [0u8; 64];
    match unsafe { narsil_mcl_g1_decompress(out.as_mut_ptr(), input.as_ptr()) } {
        0 => Ok(out),
        _ => Err(()),
    }
}

pub(crate) fn g2_compress(input: &[u8; 128]) -> Result<[u8; 64], ()> {
    if init_status() != 0 {
        return Err(());
    }
    let mut out = [0u8; 64];
    match unsafe { narsil_mcl_g2_compress(out.as_mut_ptr(), input.as_ptr()) } {
        0 => Ok(out),
        _ => Err(()),
    }
}

pub(crate) fn g2_decompress(input: &[u8; 64]) -> Result<[u8; 128], ()> {
    if init_status() != 0 {
        return Err(());
    }
    let mut out = [0u8; 128];
    match unsafe { narsil_mcl_g2_decompress(out.as_mut_ptr(), input.as_ptr()) } {
        0 => Ok(out),
        _ => Err(()),
    }
}
