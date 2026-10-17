//! Atomic JSON sidecar reads, writes, and removals.

use anyhow::{Context, Result};
use serde::{Serialize, de::DeserializeOwned};
use std::path::Path;

/// Read a JSON sidecar, treating a missing file as absent.
/// # Arguments
/// * `path` — Sidecar location.
/// # Returns
/// A decoded value, or none when the file is absent.
/// # Errors
/// Returns file-read or JSON-decoding errors.
/// # Examples
/// Use validated locations from [crate::paths] for child-agent sidecars.
pub async fn read<T: DeserializeOwned>(path: &Path) -> Result<Option<T>> {
    match tokio::fs::read(path).await {
        Ok(bytes) => serde_json::from_slice(&bytes)
            .map(Some)
            .with_context(|| format!("parse {}", path.display())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error).with_context(|| format!("read {}", path.display())),
    }
}

/// Serialize a sidecar through a temporary file and rename it into place.
/// # Arguments
/// * `path` — Final sidecar location.
/// * `value` — Serializable sidecar content.
/// # Returns
/// Unit after replacing the final file.
/// # Errors
/// Returns directory creation, serialization, write or rename errors.
/// # Examples
/// Use validated locations from [crate::paths] for child-agent sidecars.
pub async fn write<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    let tmp = path.with_extension("json.tmp");
    tokio::fs::write(&tmp, serde_json::to_vec(value)?).await?;
    if let Err(primary) = tokio::fs::rename(&tmp, path).await {
        let _ = tokio::fs::remove_file(path).await;
        tokio::fs::rename(&tmp, path).await.with_context(|| {
            format!("rename {} after initial failure: {primary}", path.display())
        })?;
    }
    Ok(())
}

/// Remove a sidecar, tolerating an already missing file.
/// # Arguments
/// * `path` — Sidecar location.
/// # Returns
/// Unit when removed or already absent.
/// # Errors
/// Returns other filesystem removal errors.
/// # Examples
/// Use validated locations from [crate::paths] for child-agent sidecars.
pub async fn remove(path: &Path) -> Result<()> {
    match tokio::fs::remove_file(path).await {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).with_context(|| format!("remove {}", path.display())),
    }
}
