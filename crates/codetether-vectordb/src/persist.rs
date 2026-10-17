//! JSON persistence for a vector store.

use super::record::Record;
use anyhow::{Context, Result};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::path::Path;

/// Load records from a JSON file, returning an empty vec if absent.
pub fn load<P>(path: &Path) -> Result<Vec<Record<P>>>
where
    P: Serialize + DeserializeOwned,
{
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content =
        std::fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    if content.trim().is_empty() {
        return Ok(Vec::new());
    }
    serde_json::from_str(&content).with_context(|| format!("parse {}", path.display()))
}

/// Persist records to a JSON file, creating parent directories as needed.
pub fn save<P>(path: &Path, records: &[Record<P>]) -> Result<()>
where
    P: Serialize + DeserializeOwned,
{
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create dir {}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(records)?;
    std::fs::write(path, json).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}
