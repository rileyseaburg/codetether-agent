//! RLM-backed summary production for [`SummaryIndex::summary_for`].
//!
//! This module is the "producer" half of step 18. The index data
//! structure lives in [`super::index`]; this file owns the async
//! call through the RLM router to materialise a summary from a
//! transcript window.

use std::sync::Arc;

use anyhow::{Context as _, Result};
use tracing::info;

use super::index::{Granularity, SummaryNode, SummaryRange};
use crate::provider::{Message, Provider};
use crate::rlm::RlmConfig;
use crate::rlm::router::AutoProcessContext;
use crate::session::helper::error::messages_to_rlm_context;

/// Produce a summary of `messages[range.start..range.end]` via RLM.
///
/// The caller slices the transcript before passing it here so this
/// function stays agnostic to the full session.
///
/// # Errors
///
/// Returns [`anyhow::Error`] if the RLM router fails.
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
    subcall_provider: Option<Arc<dyn Provider>>,
    subcall_model: Option<String>,
) -> Result<SummaryNode> {
    let slice = messages
        .get(range.start..range.end)
        .context("summary_for range out of bounds")?;

    let context = messages_to_rlm_context(slice);
    let token_hint = format!(
        "Summarise the following transcript window (~{} messages, target ≤ {} tokens). \
         Preserve key decisions, file paths, error classes, and tool outcomes.",
        slice.len(),
        target_tokens,
    );
    let input = format!("{token_hint}\n\n{context}");

    info!(
        start = range.start,
        end = range.end,
        target_tokens,
        "Producing summary via RLM"
    );

    let auto_ctx = AutoProcessContext {
        tool_id: "summary_index",
        tool_args: serde_json::json!({
            "range": [range.start, range.end],
            "target_tokens": target_tokens,
        }),
        session_id,
        abort: None,
        on_progress: None,
        provider,
        model: model.to_string(),
        bus: None,
        trace_id: None,
        subcall_provider,
        subcall_model,
    };

    let result = crate::rlm::RlmRouter::auto_process(&input, auto_ctx, rlm_config)
        .await
        .context("RLM summarisation for SummaryIndex::summary_for failed")?;

    Ok(SummaryNode {
        content: result.processed,
        target_tokens,
        granularity,
        generation,
    })
}
