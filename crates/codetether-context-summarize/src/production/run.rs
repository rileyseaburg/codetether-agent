//! Delegate captured inputs to the existing phase-summary producer.
use super::Prepared;
use codetether_context_policy::summary_index::{Granularity, SummaryNode, SummaryRange};
use codetether_provider_core::Provider;
use codetether_rlm_background::Resolver;
use codetether_summary_producer::{SummaryObservability, produce_summary};
use std::sync::Arc;
/// Produce a phase summary from captured session inputs.
/// # Arguments
/// `prepared`, `range` and `target` select the snapshot and output bound;
/// `provider`, `model` and `resolver` retain the existing model-selection policy.
/// `B` selects the application's observability bus type.
/// # Returns
/// The produced phase-summary node.
/// # Errors
/// Propagates summary-producer errors.
/// # Examples
/// ```no_run
/// # async fn example(prepared: codetether_context_summarize::production::Prepared, range: codetether_context_policy::summary_index::SummaryRange, provider: std::sync::Arc<dyn codetether_provider_core::Provider>, resolver: codetether_rlm_background::Resolver) -> anyhow::Result<()> {
/// let _node = codetether_context_summarize::production::produce::<()>(prepared, range, 100, provider, "model", resolver).await?;
/// # Ok(()) }
/// ```
pub async fn produce<B>(
    prepared: Prepared,
    range: SummaryRange,
    target: usize,
    provider: Arc<dyn Provider>,
    model: &str,
    resolver: Resolver,
) -> anyhow::Result<SummaryNode> {
    let Prepared {
        messages,
        generation,
        rlm,
        session_id,
        sub_provider,
        sub_model,
    } = prepared;
    produce_summary(
        &messages,
        range,
        target,
        Granularity::Phase,
        generation,
        provider,
        model,
        &rlm,
        &session_id,
        sub_provider,
        sub_model,
        SummaryObservability::<B>::default(),
        resolver,
    )
    .await
}
