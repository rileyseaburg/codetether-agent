//! Atomic JSON sidecar persistence.

use anyhow::Result;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::path::Path;

pub(crate) async fn read<T: DeserializeOwned>(path: &Path) -> Option<T> {
    let bytes = tokio::fs::read(path).await.ok()?;
    serde_json::from_slice(&bytes).ok()
}

pub(crate) async fn write<T: Serialize>(path: &Path, value: &T) -> Result<()> {
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
