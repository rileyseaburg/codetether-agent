//! Main entrypoint — [`produce_summary`].

use std::sync::Arc;

use anyhow::{Context as _, Result};

use super::observability::SummaryObservability;
use codetether_context_policy::summary_index::types::{Granularity, SummaryNode, SummaryRange};
use codetether_provider_core::{Message, Provider};
use codetether_rlm::RlmConfig;

/// Produce a summary of `messages[range.start..range.end]` via RLM.
///
/// # Errors
///
/// Returns an error when the requested transcript range is out of bounds.
/// Background summary misses use deterministic chunk compression.
/// # Arguments
/// * `messages`, `range` — Transcript and half-open range to summarize.
/// * `target_tokens`, `granularity`, `generation` — Output bounds and index metadata.
/// * `provider`, `model`, `rlm_config`, `session_id` — Background summary context.
/// * `_subcall_provider`, `_subcall_model`, `_observability` — Retained compatibility inputs.
/// * `resolver` — Native model-selection callback for the background worker.
/// # Returns
/// A summary-index node containing a cached summary or deterministic fallback.
/// # Examples
/// ```no_run
/// # async fn produce(provider: std::sync::Arc<dyn codetether_provider_core::Provider>, resolver: codetether_rlm_background::Resolver, range: codetether_context_policy::summary_index::SummaryRange, granularity: codetether_context_policy::summary_index::Granularity) {
/// let _result = codetether_summary_producer::produce_summary(&[], range, 100, granularity, 0, provider, "model", &Default::default(), "session", None, None, codetether_summary_producer::SummaryObservability::<()>::default(), resolver).await;
/// # }
/// ```
#[allow(clippy::too_many_arguments)]
pub async fn produce_summary<B>(
    messages: &[Message],
    range: SummaryRange,
    target_tokens: usize,
    granularity: Granularity,
    generation: u64,
    provider: Arc<dyn Provider>,
    model: &str,
    rlm_config: &RlmConfig,
    session_id: &str,
    _subcall_provider: Option<Arc<dyn Provider>>,
    _subcall_model: Option<String>,
    _observability: SummaryObservability<B>,
    resolver: codetether_rlm_background::Resolver,
) -> Result<SummaryNode> {
    let slice = messages
        .get(range.start..range.end)
        .context("summary_for range out of bounds")?;
    let provenance = codetether_context_policy::summary_index::SummaryProvenance::new(
        slice,
        "background-or-fallback",
        Some(model),
    )?;
    let input = super::request::format(slice, target_tokens, &provenance);
    let content = super::background::summary_or_fallback(
        &input,
        target_tokens,
        provider,
        model,
        rlm_config,
        session_id,
        resolver,
    );
    Ok(SummaryNode {
        content,
        target_tokens,
        granularity,
        generation,
        provenance: Some(provenance),
    })
}
