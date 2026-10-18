//! Read and validate a prepared-index sidecar.
use crate::Prepared;
use std::path::Path;
const SCHEMA_VERSION: u8 = 1;

/// Decode JSON and accept only the supported schema version.
/// # Returns
/// None for invalid JSON or an unsupported schema.
pub fn decode(bytes: &[u8]) -> Option<Prepared> {
    let prepared: Prepared = serde_json::from_slice(bytes).ok()?;
    (prepared.schema_version == SCHEMA_VERSION).then_some(prepared)
}
/// Read a sidecar, returning None on path, read or decoding failure.
pub async fn read(path: &Path) -> Option<Prepared> {
    let bytes = tokio::fs::read(path).await.ok()?;
    decode(&bytes)
}
