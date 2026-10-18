//! Shared typed writeback journal with native session-path resolution.
pub use codetether_writeback_journal::{JournalEntry, Op, RejectReason, TxnId, WritebackJournal};
/// Append ordered journal events beside the validated native session path.
/// # Errors
/// Returns session-path validation, filesystem or serialization errors.
pub async fn append_entries(session_id: &str, entries: &[JournalEntry]) -> anyhow::Result<()> {
    let path = crate::session::Session::session_path(session_id)?.with_extension("journal.jsonl");
    codetether_writeback_journal::append_entries(&path, entries).await
}
