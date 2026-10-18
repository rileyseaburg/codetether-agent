//! Thread id validation and path construction.

use anyhow::{Result, bail};
use std::path::{Path, PathBuf};

const MAX_THREAD_ID_LEN: usize = 128;

pub(super) fn thread_path(root: &Path, thread_id: &str) -> Result<PathBuf> {
    ensure_safe_thread_id(thread_id)?;
    Ok(root.join(format!("{thread_id}.jsonl")))
}

fn ensure_safe_thread_id(thread_id: &str) -> Result<()> {
    if thread_id.is_empty() || thread_id.len() > MAX_THREAD_ID_LEN {
        bail!("Invalid thread ID: rejecting path traversal risk");
    }
    if !thread_id.bytes().all(is_safe_thread_id_byte) {
        bail!("Invalid thread ID: rejecting path traversal risk");
    }
    Ok(())
}

fn is_safe_thread_id_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_'
}
