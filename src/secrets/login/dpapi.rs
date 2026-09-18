//! Current-user DPAPI sealing; no machine-wide key or interactive elevation.

use anyhow::{Result, ensure};
use windows::Win32::Foundation::{HLOCAL, LocalFree};
use windows::Win32::Security::Cryptography::CRYPT_INTEGER_BLOB;
#[path = "dpapi_call.rs"]
mod call;

pub(super) fn transform(bytes: &[u8], protect: bool) -> Result<Vec<u8>> {
    ensure!(
        bytes.len() <= u32::MAX as usize,
        "Vault profile is too large"
    );
    let input = CRYPT_INTEGER_BLOB {
        cbData: bytes.len() as u32,
        pbData: bytes.as_ptr().cast_mut(),
    };
    let output = call::invoke(&input, protect)?;
    ensure!(!output.pbData.is_null(), "DPAPI returned no profile data");
    // SAFETY: a successful DPAPI call returns cbData readable bytes.
    let value =
        unsafe { std::slice::from_raw_parts(output.pbData, output.cbData as usize).to_vec() };
    // SAFETY: the returned allocation remains valid until LocalFree.
    unsafe {
        std::ptr::write_bytes(output.pbData, 0, output.cbData as usize);
        let _ = LocalFree(Some(HLOCAL(output.pbData.cast())));
    }
    Ok(value)
}
