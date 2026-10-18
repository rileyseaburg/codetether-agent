//! Main entrypoint — [`produce_summary`].

use std::sync::Arc;

use anyhow::Result;

use codetether_context_policy::summary_index::types::{Granularity, SummaryNode, SummaryRange};
use codetether_provider_core::{Message, Provider};
use codetether_rlm::RlmConfig;
pub use codetether_summary_producer::SummaryObservability;

/// Produce a summary of `messages[range.start..range.end]` via RLM.
///
/// # Errors
///
/// Returns the shared producer's range or summary-generation error.
/// # Arguments
/// The transcript range, token target, generation and granularity describe the
/// summary. Provider/model/RLM settings select the native background worker;
/// session and optional subcall/observation values retain caller context.
/// # Returns
/// A summary node with the existing cached or deterministic fallback content.
/// # Examples
/// ```no_run
/// # async fn example(provider: std::sync::Arc<dyn codetether_provider_core::Provider>, range: codetether_context_policy::summary_index::SummaryRange, granularity: codetether_context_policy::summary_index::Granularity) {
/// let result = codetether_native_proactive::summary::produce_summary(
///     &[], range, 100, granularity, 0, provider, "model", &Default::default(),
///     "session", None, None, codetether_native_proactive::summary::SummaryObservability::<()>::default(),
/// ).await;
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
) -> Result<SummaryNode> {
    codetether_summary_producer::produce_summary(
        messages,
        range,
        target_tokens,
        granularity,
        generation,
        provider,
        model,
        rlm_config,
        session_id,
        _subcall_provider,
        _subcall_model,
        _observability,
        crate::Background::<()>::resolve_model,
    )
    .await
}
