//! Atomic JSON sidecar persistence.

use anyhow::Result;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::path::Path;

/// Read and decode a JSON file without surfacing I/O or decoding errors.
/// # Arguments
/// Receives the JSON path and inferred deserialization type.
/// # Returns
/// The decoded value, or None.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let value: Option<serde_json::Value> = codetether_recall_store::atomic::read(std::path::Path::new("index.json")).await;
/// # }
/// ```
pub async fn read<T: DeserializeOwned>(path: &Path) -> Option<T> {
    let bytes = tokio::fs::read(path).await.ok()?;
    serde_json::from_slice(&bytes).ok()
}

/// Serialize JSON to a temporary sibling and rename it into place.
/// # Arguments
/// Receives a destination path and serializable value.
/// # Returns
/// Completes after replacement, retrying rename after removing the destination.
/// # Errors
/// Propagates directory, serialization, write and final rename errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// codetether_recall_store::atomic::write(std::path::Path::new("index.json"), &vec!["id"]).await
/// # }
/// ```
pub async fn write<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    let tmp = path.with_extension("json.tmp");
    tokio::fs::write(&tmp, serde_json::to_vec(value)?).await?;
    if let Err(primary) = tokio::fs::rename(&tmp, path).await {
        let _ = tokio::fs::remove_file(path).await;
        tokio::fs::rename(&tmp, path)
            .await
            .map_err(|retry| anyhow::anyhow!("recall rename failed: {primary} (retry: {retry})"))?;
    }
    Ok(())
}
