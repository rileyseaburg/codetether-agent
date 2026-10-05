//! Append-only JSON Lines log of goal verifier verdicts.
//!
//! Stored beside the session task log as `<session>.verdicts.jsonl`, so a
//! goal's completion can be audited after the fact: which verifier decided,
//! when, and on which report.

#[path = "verdict_record.rs"]
mod record;

pub use record::VerdictRecord;

use anyhow::{Context, Result};
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;

/// Path of the verdict log for `session_id`.
///
/// # Errors
///
/// Fails when the data directory cannot be determined.
pub fn path(session_id: &str) -> Result<PathBuf> {
    let tasks = crate::session::tasks::task_log_path(session_id)?;
    Ok(tasks.with_file_name(format!("{session_id}.verdicts.jsonl")))
}

/// Append `record` to the session's verdict log.
///
/// # Errors
///
/// Fails when the log cannot be opened or written.
pub async fn append(session_id: &str, record: &VerdictRecord) -> Result<()> {
    let path = path(session_id)?;
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await.ok();
    }
    let mut line = serde_json::to_string(record).context("serialize verdict")?;
    line.push('\n');
    let mut file = tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .await
        .with_context(|| format!("open {}", path.display()))?;
    file.write_all(line.as_bytes()).await?;
    Ok(())
}

/// Read every verdict recorded for `session_id`; missing log means none.
///
/// # Errors
///
/// Fails when an existing log cannot be read.
pub async fn read(session_id: &str) -> Result<Vec<VerdictRecord>> {
    let path = path(session_id)?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text = tokio::fs::read_to_string(&path).await?;
    Ok(text
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect())
}
