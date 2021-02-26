//! This module contains rust adapters to the underlying RNG primitives
#[cfg(feature = "getrandom")]
pub fn optee_getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    unsafe { crate::wrapper::raw::TEE_GenerateRandom(buf.as_mut_ptr() as _, buf.len() as _) }

    Ok(())
}
