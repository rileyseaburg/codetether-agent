//! Main entrypoint — [`produce_summary`].

use std::sync::Arc;

use anyhow::{Context as _, Result};

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
    let slice = messages
        .get(range.start..range.end)
        .context("summary_for range out of bounds")?;
    let context = crate::session::helper::error::messages_to_rlm_context(slice);
    let input = format!(
        "Summarise the following transcript window (~{} messages, target ≤ {} tokens). \
         Preserve key decisions, file paths, error classes, and tool outcomes.\n\n{context}",
        slice.len(),
        target_tokens,
    );
    let content = super::background::summary_or_fallback(
        &input,
        target_tokens,
        provider,
        model,
        rlm_config,
        session_id,
    );
    Ok(SummaryNode {
        content,
        target_tokens,
        granularity,
        generation,
    })
}
