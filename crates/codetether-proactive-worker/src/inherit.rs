//! Reuse prefix-compatible summaries from the previous prepared sidecar.

use codetether_context_policy::summary_index::SummaryIndex;

use codetether_prepared_context as freshness;
use codetether_proactive_runtime::Snapshot;

pub(super) async fn index<H: crate::Host>(snapshot: &Snapshot, host: &H) -> SummaryIndex {
    let Some(previous) = host.read_prepared(&snapshot.session_id).await else {
        return snapshot.index.clone();
    };
    if freshness::matches(
        &snapshot.messages,
        previous.message_count,
        previous.fingerprint,
    ) {
        previous.index
    } else {
        snapshot.index.clone()
    }
}
