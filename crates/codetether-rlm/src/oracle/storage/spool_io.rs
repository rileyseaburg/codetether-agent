//! Spool-file naming and atomic-write primitives.
//!
//! These are low-level disk operations used by the storage
//! manager to safely persist `.jsonl` trace records without
//! risking partial writes.
//!
//! # Examples
//!
//! ```ignore
//! let name = spool_filename(&record, 1712345678000);
//! // => "1712345678000_<trace_id>_golden.jsonl"
//! ```

use super::super::record::OracleTraceRecord;
use anyhow::Result;
use std::path::Path;

/// Build a collision-resistant spool filename.
///
/// Embeds the millisecond timestamp, trace ID (or a random UUID
/// when the record has none), and the verdict label so files can
/// be visually triaged on disk.
///
/// # Examples
///
/// ```ignore
/// let name = spool_filename(&record, 1712345678000);
/// assert!(name.ends_with(".jsonl"));
/// assert!(name.contains("golden"));
/// ```
pub fn spool_filename(record: &OracleTraceRecord, ts_ms: i64) -> String {
    let trace_id = if record.trace.trace_id.is_empty() {
        uuid::Uuid::new_v4().to_string()
    } else {
        record.trace.trace_id.clone()
    };
    format!("{ts_ms}_{trace_id}_{}.jsonl", record.verdict)
}

/// Persist `value` to `path` without partial-write risk.
///
/// Writes to a hidden `.tmp` sibling first, then atomically
/// renames into place so concurrent readers never observe a
/// half-written file.
///
/// # Examples
///
/// ```ignore
/// write_json_atomic(Path::new("/tmp/trace.jsonl"), &record).await?;
/// ```
pub async fn write_json_atomic(path: &Path, value: &OracleTraceRecord) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid spool path {}", path.display()))?;
    tokio::fs::create_dir_all(parent).await?;
    let tmp_path = parent.join(format!(
        ".{}.tmp",
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("oracle_record")
    ));
    let json = serde_json::to_string(value)
        .map_err(|e| anyhow::anyhow!("Failed to serialize oracle record: {e}"))?;
    tokio::fs::write(&tmp_path, json).await?;
    tokio::fs::rename(&tmp_path, path).await?;
    Ok(())
}
