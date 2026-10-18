//! Capture the source checkpoint before persisting a manual summary.
use crate::Snapshot;
use codetether_context_policy::summary_index::{SummaryNode, SummaryRange};

/// Persist a manual summary using the snapshot's current source revision.
/// # Arguments
/// `snapshot` supplies the checkpoint; `range` and `node` describe the summary.
/// # Returns
/// Unit after the existing revision-checked summary transaction succeeds.
/// # Errors
/// Propagates serialization, revision conflict, database and worker errors.
/// # Panics
/// Panics on checkpoint poisoning or when no Tokio runtime is available.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_session_store::Snapshot + Sync>(snapshot: &S, range: codetether_context_policy::summary_index::SummaryRange, node: &codetether_context_policy::summary_index::SummaryNode) -> anyhow::Result<()> {
/// codetether_session_store::summaries::put_snapshot(snapshot, range, node).await?;
/// # Ok(()) }
/// ```
pub async fn put_snapshot(
    snapshot: &(impl Snapshot + Sync),
    range: SummaryRange,
    node: &SummaryNode,
) -> anyhow::Result<()> {
    let state = snapshot.state().0.lock().unwrap().clone();
    super::put(state, range, node).await
}
