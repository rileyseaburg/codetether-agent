//! Append-only verdict journal at a caller-resolved path.
use crate::VerdictRecord;
use anyhow::{Context, Result};
use std::path::Path;
use tokio::io::AsyncWriteExt;

/// Append `record` to the verdict log at `path`.
///
/// # Errors
///
/// Fails when the log cannot be opened or written.
/// # Arguments
/// * `path` - Caller-resolved verdict journal path.
/// * `record` - Verdict to append as one JSON line.
/// # Returns
/// Success after appending the serialized record.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let record = codetether_goal_verification::VerdictRecord::new("goal", "complete", true, "model", "VERDICT: PASS");
/// codetether_goal_verification::journal::append(std::path::Path::new("verdicts.jsonl"), &record).await?;
/// # Ok(()) }
/// ```
pub async fn append(path: &Path, record: &VerdictRecord) -> Result<()> {
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

/// Read every verdict at `path`; a missing log means none.
///
/// # Errors
///
/// Fails when an existing log cannot be read.
/// # Arguments
/// * `path` - Caller-resolved verdict journal path.
/// # Returns
/// Parsed records, ignoring malformed lines; empty when the file is absent.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let _records = codetether_goal_verification::journal::read(std::path::Path::new("verdicts.jsonl")).await?;
/// # Ok(()) }
/// ```
pub async fn read(path: &Path) -> Result<Vec<VerdictRecord>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text = tokio::fs::read_to_string(&path).await?;
    Ok(text
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect())
}
