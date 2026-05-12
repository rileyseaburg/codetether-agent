//! Sanity size cap for [`Session::load`](super::super::Session::load).
//!
//! `tokio::fs::read_to_string` preallocates a `String` whose capacity is
//! the file's reported size. A corrupted/runaway session file claiming
//! tens of gigabytes would OOM the process before serde sees a byte.
//! Refuse anything past [`MAX_SESSION_FILE_BYTES`].

use anyhow::{Result, bail};
use std::path::Path;
use tokio::fs;

/// Maximum allowed on-disk size for a session JSON file.
pub const MAX_SESSION_FILE_BYTES: u64 = 512 * 1024 * 1024; // 512 MiB

/// Bail if `path` exists and exceeds [`MAX_SESSION_FILE_BYTES`]. Missing
/// or unstattable files are silently allowed through — the subsequent
/// read will produce its own diagnostic.
pub async fn ensure_session_file_size(path: &Path) -> Result<()> {
    if let Ok(meta) = fs::metadata(path).await
        && meta.len() > MAX_SESSION_FILE_BYTES
    {
        bail!(
            "session file {} is {} bytes, exceeds {} MiB cap; refusing to load to avoid OOM",
            path.display(),
            meta.len(),
            MAX_SESSION_FILE_BYTES / (1024 * 1024),
        );
    }
    Ok(())
}
