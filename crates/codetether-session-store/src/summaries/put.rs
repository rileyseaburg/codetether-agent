//! Persist an explicit summary only against its source session revision.
use anyhow::Result;
use codetether_context_policy::summary_index::{SummaryNode, SummaryRange};
use rusqlite::{TransactionBehavior, params};

/// Persist a manual summary against the captured source revision.
/// # Arguments
/// `state` is a cloned checkpoint; `range` and `node` describe the summary.
/// # Returns
/// Unit when the revision check and write commit succeed.
/// # Errors
/// Serialization, revision conflict, SQLite operations or the worker fail.
/// # Panics
/// Requires a Tokio runtime for the blocking worker.
/// # Examples
/// ```rust,no_run
/// # async fn example(state: codetether_session_store::Checkpoint, range: codetether_context_policy::summary_index::SummaryRange, node: &codetether_context_policy::summary_index::SummaryNode) -> anyhow::Result<()> {
/// let result = codetether_session_store::summaries::put(state, range, node).await?;
/// # Ok(()) }
/// ```
pub async fn put(state: crate::Checkpoint, range: SummaryRange, node: &SummaryNode) -> Result<()> {
    let body = serde_json::to_string(node)?;
    tokio::task::spawn_blocking(move || {
        let mut db = crate::connection::open(&state.path)?;
        // Check the source revision only after obtaining SQLite's writer reservation.
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let revision: i64 = tx.query_row("SELECT revision FROM sessions WHERE id=?1", [&state.id], |r| r.get(0))?;
        anyhow::ensure!(revision == state.revision, "SESSION_REVISION_CONFLICT: summary source changed");
        tx.execute("INSERT INTO projections VALUES (?1,'manual_summary',?2,?3,?4)
            ON CONFLICT(session_id,name,seq) DO UPDATE SET end_seq=excluded.end_seq,body=excluded.body",
            params![state.id,(state.message_start+range.start) as i64,(state.message_start+range.end) as i64,body])?;
        tx.commit()?;
        anyhow::Ok(())
    }).await?
}
