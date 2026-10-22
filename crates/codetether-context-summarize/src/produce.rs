//! Produce a missing range through its existing producer, then persist the cache.
use crate::SummarySource;
use anyhow::Result;
use codetether_context_policy::summary_index::{SummaryNode, SummaryRange};
use codetether_provider_core::Provider;
use std::sync::Arc;
pub(super) fn validate_cache<S: SummarySource>(session: &mut S) {
    let mut validated = session.summary_index().clone();
    validated.retain_valid(session.messages());
    *session.summary_index_mut() = validated;
}
pub(super) async fn produce_cached<S: SummarySource>(
    session: &mut S,
    range: SummaryRange,
    target: usize,
    provider: Arc<dyn Provider>,
    model: &str,
) -> Result<SummaryNode> {
    validate_cache(session);
    let prepared = session.prepare();
    let node = session
        .summary_index_mut()
        .summary_for(range, |r| S::produce(prepared, r, target, provider, model))
        .await?;
    anyhow::ensure!(
        node.valid_for(session.messages(), range),
        "Summary producer returned invalid source or policy provenance"
    );
    session.persist_summary(range, &node).await?;
    Ok(node)
}
