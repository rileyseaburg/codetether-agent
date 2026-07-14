//! Tolerant discovery-record scanning.

use anyhow::{Context, Result};

use super::MuxRecord;

pub(in crate::mux) async fn list() -> Result<Vec<MuxRecord>> {
    let mut records = Vec::new();
    let Ok(mut entries) = tokio::fs::read_dir(super::path::root()?).await else {
        return Ok(records);
    };
    while let Some(entry) = entries.next_entry().await.context("scan mux registry")? {
        if entry.path().extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let bytes = tokio::fs::read(entry.path())
            .await
            .context("read mux record")?;
        if let Ok(record) = serde_json::from_slice(&bytes) {
            records.push(record);
        }
    }
    records.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(records)
}
