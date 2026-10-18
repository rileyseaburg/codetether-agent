//! Scan all small registry records in one blocking operation.
use super::super::MuxRecord;
use anyhow::{Context, Result};
pub(super) fn run(root: &std::path::Path) -> Result<Vec<MuxRecord>> {
    let mut records = Vec::new();
    let Ok(entries) = std::fs::read_dir(root) else {
        return Ok(records);
    };
    for entry in entries {
        let path = entry.context("scan mux registry")?.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let bytes = std::fs::read(&path).context("read mux record")?;
        if let Ok(record) = super::super::io::decode(&bytes) {
            records.push(record);
        }
    }
    records.sort_by(|left, right| left.key.cmp(&right.key));
    Ok(records)
}
