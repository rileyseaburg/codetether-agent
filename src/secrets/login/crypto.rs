//! Storage protection boundary: DPAPI on Windows, private file permissions on Unix.

#[cfg(windows)]
#[path = "dpapi.rs"]
mod dpapi;

pub(super) fn encode(bytes: &[u8]) -> anyhow::Result<Vec<u8>> {
    #[cfg(windows)]
    return dpapi::transform(bytes, true);
    #[cfg(not(windows))]
    Ok(bytes.to_vec())
}

pub(super) fn decode(bytes: &[u8]) -> anyhow::Result<Vec<u8>> {
    #[cfg(windows)]
    return dpapi::transform(bytes, false);
    #[cfg(not(windows))]
    Ok(bytes.to_vec())
}
