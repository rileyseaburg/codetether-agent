//! Atomic owner-private mux registry I/O keyed by server.

use anyhow::{Context, Result};

use super::MuxRecord;

pub(in crate::mux) async fn store(record: &MuxRecord) -> Result<()> {
    let root = super::path::root()?;
    tokio::fs::create_dir_all(&root)
        .await
        .context("create mux registry")?;
    super::permissions::owner_only_dir(&root)?;
    let path = super::path::record(&record.key)?;
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

/// Load the server record stored under `key`.
pub(in crate::mux) async fn load_key(key: &str) -> Result<MuxRecord> {
    let bytes = tokio::fs::read(super::path::record(key)?)
        .await
        .context("read mux record")?;
    decode(&bytes)
}

/// Decode a record, accepting pre-multi-session files that used `name`.
pub(super) fn decode(bytes: &[u8]) -> Result<MuxRecord> {
    if let Ok(record) = serde_json::from_slice::<MuxRecord>(bytes) {
        return Ok(record);
    }
    let legacy: serde_json::Value = serde_json::from_slice(bytes).context("decode mux record")?;
    let mut value = legacy.clone();
    if let (Some(object), Some(name)) = (value.as_object_mut(), legacy["name"].as_str()) {
        object.insert("key".into(), serde_json::Value::String(name.to_string()));
    }
    serde_json::from_value(value).context("decode legacy mux record")
}

pub(in crate::mux) async fn remove_key(key: &str) -> Result<()> {
    match tokio::fs::remove_file(super::path::record(key)?).await {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).context("remove mux record"),
    }
}
