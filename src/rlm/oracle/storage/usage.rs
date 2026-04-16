//! Disk-usage accounting for the local spool directory.
//!
//! Provides byte-level size calculations so the storage manager
//! can warn or reject when the spool exceeds its budget.
//!
//! # Examples
//!
//! ```ignore
//! let bytes = spool_usage_bytes(Path::new("/tmp/spool")).await?;
//! println!("spool uses {bytes} bytes");
//! ```

use anyhow::Result;
use std::path::Path;

/// Sum the byte sizes of all `.jsonl` files in `dir`.
///
/// Returns `0` when the directory does not exist rather than
/// failing, so callers can check usage before the spool is
/// first created.
///
/// # Examples
///
/// ```ignore
/// let bytes = spool_usage_bytes(&spool_dir).await?;
/// if bytes > max_bytes {
///     tracing::warn!("spool over budget");
/// }
/// ```
pub async fn spool_usage_bytes(dir: &Path) -> Result<u64> {
    if !dir.exists() {
        return Ok(0);
    }
    let mut total = 0u64;
    let mut entries = tokio::fs::read_dir(dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("jsonl") {
            if let Ok(meta) = entry.metadata().await {
                total = total.saturating_add(meta.len());
            }
        }
    }
    Ok(total)
}
