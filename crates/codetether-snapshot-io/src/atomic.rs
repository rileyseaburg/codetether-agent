//! Isolated, durable snapshot replacement; never unlink the previous snapshot.

use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};

/// # Arguments
/// `path` selects the destination; `content` is the complete replacement.
/// The caller must create the parent directory first.
/// # Returns
/// The same owned bytes after successful durable replacement.
/// # Errors
/// Returns temporary-file, write, sync, replacement or worker errors.
/// # Panics
/// Requires a Tokio runtime for the blocking worker.
/// # Examples
/// ```rust,no_run
/// # async fn example(path: &std::path::Path) -> anyhow::Result<()> {
/// let bytes = codetether_snapshot_io::atomic_write(path, b"snapshot".to_vec()).await?;
/// assert_eq!(bytes, b"snapshot");
/// # Ok(()) }
/// ```
pub async fn atomic_write(path: &Path, content: Vec<u8>) -> Result<Vec<u8>> {
    let path = path.to_path_buf();
    tokio::task::spawn_blocking(move || {
        let parent = path.parent().context("session snapshot has no parent")?;
        let mut temp = tempfile::Builder::new()
            .prefix(".session-")
            .tempfile_in(parent)
            .context("create unique session temporary file")?;
        temp.write_all(&content).context("write session snapshot")?;
        temp.as_file().sync_all().context("sync session snapshot")?;
        // A failed atomic replacement must not remove the old snapshot.
        temp.persist(&path).context("replace session snapshot")?;
        #[cfg(unix)]
        std::fs::File::open(parent)?.sync_all()?;
        Ok(content)
    })
    .await
    .context("session snapshot writer panicked")?
}

#[cfg(test)]
#[path = "atomic/tests.rs"]
mod tests;
