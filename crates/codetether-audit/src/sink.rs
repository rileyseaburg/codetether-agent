//! Best-effort append-only JSONL persistence, independent of memory retention.

use crate::AuditEntry;
use std::path::Path;
use tokio::io::AsyncWriteExt;

pub(crate) async fn append(path: &Path, entry: &AuditEntry) {
    if let Ok(line) = serde_json::to_string(entry)
        && let Ok(mut file) = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .await
    {
        let _ = file.write_all(line.as_bytes()).await;
        let _ = file.write_all(b"\n").await;
    }
}
