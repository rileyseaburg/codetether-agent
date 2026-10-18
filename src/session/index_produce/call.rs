//! Main entrypoint — [`produce_summary`].

use std::sync::Arc;

use anyhow::Result;

use super::super::index::types::{Granularity, SummaryNode, SummaryRange};
use super::observability::SummaryObservability;
use crate::provider::{Message, Provider};
use crate::rlm::RlmConfig;

/// Produce a summary of `messages[range.start..range.end]` via RLM.
///
/// # Errors
///
/// Returns [`anyhow::Error`] if the RLM router fails.
#[allow(clippy::too_many_arguments)]
pub async fn produce_summary(
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
    _observability: SummaryObservability,
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
        crate::session::helper::rlm_background::resolve_model,
    )
    .await
}
