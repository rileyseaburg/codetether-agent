//! Atomic owner-private mux registry I/O.

use anyhow::{Context, Result};

use super::MuxRecord;

pub(in crate::mux) async fn store(record: &MuxRecord) -> Result<()> {
    let root = super::path::root()?;
    tokio::fs::create_dir_all(&root)
        .await
        .context("create mux registry")?;
    super::permissions::owner_only_dir(&root)?;
    let path = super::path::record(&record.name)?;
    let temp = path.with_extension(format!("{}.tmp", std::process::id()));
    let bytes = serde_json::to_vec_pretty(record).context("encode mux record")?;
    tokio::fs::write(&temp, bytes)
        .await
        .context("write mux record")?;
    super::permissions::owner_only_file(&temp)?;
    tokio::fs::rename(&temp, &path)
        .await
        .context("publish mux record")
}

pub(in crate::mux) async fn load(name: &str) -> Result<MuxRecord> {
    let bytes = tokio::fs::read(super::path::record(name)?)
        .await
        .context("read mux record")?;
    serde_json::from_slice(&bytes).context("decode mux record")
}

pub(in crate::mux) async fn remove(name: &str) -> Result<()> {
    match tokio::fs::remove_file(super::path::record(name)?).await {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).context("remove mux record"),
    }
}
