//! Preserve bounded legacy summary caches during the one-time import.
use anyhow::Result;
use codetether_context_policy::summary_index::SummaryIndex;
use std::path::Path;
/// Preserve legacy summary nodes without replacing existing rows.
/// # Arguments
/// `path` locates storage, `id` identifies the session, and `index` holds nodes.
/// # Returns
/// Unit after the import transaction commits.
/// # Errors
/// Opening storage, serializing nodes or committing the import fails.
/// # Examples
/// ```rust,no_run
/// # fn example(path: &std::path::Path, index: &codetether_context_policy::summary_index::SummaryIndex) -> anyhow::Result<()> {
/// codetether_session_store::summaries::import(path, "session-123", index)?;
/// # Ok(()) }
/// ```
pub fn import(path: &Path, id: &str, index: &SummaryIndex) -> Result<()> {
    let mut rows = Vec::new();
    for (range, node) in index.entries() {
        rows.push((
            range.start as i64,
            range.end as i64,
            serde_json::to_string(node)?,
        ));
    }
    let id = id.to_owned();
    crate::coordinator::write(path, move |db| {
        let tx = db.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        for (start, end, body) in rows {
            tx.execute(
                "INSERT OR IGNORE INTO projections VALUES (?1,'manual_summary',?2,?3,?4)",
                rusqlite::params![id, start, end, body],
            )?;
        }
        tx.commit()?;
        Ok(())
    })
}
