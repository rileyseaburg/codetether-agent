//! Merge fresh prepared and local summaries without trusting stale provenance.
use codetether_context_policy::summary_index::SummaryIndex;
pub(super) async fn index(session: &impl crate::Source) -> SummaryIndex {
    let mut prepared = session.prepared_index().await;
    prepared.retain_valid(session.messages());
    if let Some(local) = session.summary_index() {
        for (range, node) in local.entries() {
            if node.valid_for(session.messages(), *range) {
                prepared.insert(*range, node.clone());
            }
        }
    }
    prepared
}
