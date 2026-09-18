//! Minimal Windows DPAPI call boundary; the caller frees returned memory.

use anyhow::Result;
use windows::Win32::Security::Cryptography::{
    CRYPT_INTEGER_BLOB, CRYPTPROTECT_UI_FORBIDDEN, CryptProtectData, CryptUnprotectData,
};

pub(super) fn invoke(input: &CRYPT_INTEGER_BLOB, protect: bool) -> Result<CRYPT_INTEGER_BLOB> {
    let mut output = CRYPT_INTEGER_BLOB::default();
    // SAFETY: the caller supplies a live input buffer; output starts empty.
    let result = unsafe {
        if protect {
            CryptProtectData(
                input,
                windows::core::PCWSTR::null(),
                None,
                None,
                None,
                CRYPTPROTECT_UI_FORBIDDEN,
                &mut output,
            )
        } else {
            CryptUnprotectData(
                input,
                None,
                None,
                None,
                None,
                CRYPTPROTECT_UI_FORBIDDEN,
                &mut output,
            )
        }
    };
    result
        .map_err(|_| anyhow::anyhow!("Windows could not protect/read this user's Vault profile"))?;
    Ok(output)
}
