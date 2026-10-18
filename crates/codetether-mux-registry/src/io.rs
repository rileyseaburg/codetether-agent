//! Atomic owner-private mux registry I/O keyed by server.

use anyhow::{Context, Result};

use super::MuxRecord;

/// Atomically publish a server record with owner-only permissions.
/// # Arguments
/// `record` supplies the registry key and connection/state data.
/// # Returns
/// Unit after the temporary file has been renamed into place.
/// # Errors
/// Reports directory, permissions, encoding, write or rename failures.
/// # Examples
/// ```no_run
/// # async fn example(record: &codetether_mux_registry::MuxRecord) -> anyhow::Result<()> {
/// codetether_mux_registry::store(record).await?;
/// # Ok(()) }
/// ```
pub async fn store(record: &MuxRecord) -> Result<()> {
    let root = super::path::root()?;
    std::fs::create_dir_all(&root).context("create mux registry")?;
    super::permissions::owner_only_dir(&root)?;
    let path = super::path::record(&record.key)?;
    let temp = path.with_extension(format!("{}.tmp", std::process::id()));
    let bytes = serde_json::to_vec_pretty(record).context("encode mux record")?;
    std::fs::write(&temp, bytes).context("write mux record")?;
    super::permissions::owner_only_file(&temp)?;
    std::fs::rename(&temp, &path).context("publish mux record")
}

/// Load the server record stored under `key`.
/// Load the discovery record for an exact server key.
/// # Arguments
/// `key` must be a valid registry name.
/// # Returns
/// The decoded current or legacy record.
/// # Errors
/// Reports invalid names, read failures or invalid record JSON.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let _record = codetether_mux_registry::load_key("ws-example").await?;
/// # Ok(()) }
/// ```
pub async fn load_key(key: &str) -> Result<MuxRecord> {
    let bytes = std::fs::read(super::path::record(key)?).context("read mux record")?;
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

/// Remove a discovery record, treating an absent file as success.
/// # Arguments
/// `key` is the validated server record name.
/// # Returns
/// Unit after removal or when the record was already absent.
/// # Errors
/// Reports invalid names and filesystem failures other than not-found.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// codetether_mux_registry::remove_key("ws-example").await?;
/// # Ok(()) }
/// ```
pub async fn remove_key(key: &str) -> Result<()> {
    match std::fs::remove_file(super::path::record(key)?) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).context("remove mux record"),
    }
}
