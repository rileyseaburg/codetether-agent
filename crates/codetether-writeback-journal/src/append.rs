//! Append one ordered JSONL batch to the resolved journal location.
use crate::JournalEntry;
use tokio::io::AsyncWriteExt;

/// Append journal entries to `<session-id>.journal.jsonl`.
///
/// Best-effort durability helper used by lifecycle call sites such as
/// `Session::save`. The journal remains append-only: callers hand us a
/// pre-built ordered slice and we stream it as JSONL.
/// # Arguments
/// `path` is a validated, caller-resolved journal path; `entries` is the
/// ordered batch to append. The application owns session-ID validation.
/// # Returns
/// Unit after writing the batch and flushing the file.
/// # Errors
/// Propagates directory, file-open, serialization, write and flush failures.
/// # Examples
/// ```rust,no_run
/// # async fn example(path: &std::path::Path) -> anyhow::Result<()> {
/// let mut journal = codetether_writeback_journal::WritebackJournal::new("session");
/// journal.stage(codetether_writeback_journal::Op::Save);
/// codetether_writeback_journal::append_entries(path, journal.entries()).await?;
/// # Ok(()) }
/// ```
pub async fn append_entries(
    path: &std::path::Path,
    entries: &[JournalEntry],
) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    let mut file = tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .await?;
    let mut batch = Vec::new();
    for entry in entries {
        let line = serde_json::to_string(entry)?;
        batch.extend_from_slice(line.as_bytes());
        batch.push(b'\n');
    }
    if !batch.is_empty() {
        file.write_all(&batch).await?;
    }
    file.flush().await?;
    Ok(())
}
