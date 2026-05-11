//! Main entrypoint — [`produce_summary`].

use std::sync::Arc;

use anyhow::{Context as _, Result};
use tracing::info;
use uuid::Uuid;

use super::super::index::types::{Granularity, SummaryNode, SummaryRange};
use super::build_context::{SummaryProduceParams, build_auto_context};
use crate::provider::{Message, Provider};
use crate::rlm::RlmConfig;
use crate::session::SessionBus;

/// Optional observability handle for [`produce_summary`].
///
/// Defaults to "no bus, no trace id" so existing call sites don't need
/// to change. Production callers (e.g. `derive_incremental`,
/// `derive_reset`) should construct one wrapping their parent
/// `event_tx` so the resulting `RlmProgress`/`RlmComplete` events ride
/// the same trace id as the surrounding `CompactionStarted`/`Completed`
/// pair.
#[derive(Default)]
pub struct SummaryObservability {
    pub bus: Option<SessionBus>,
    pub trace_id: Option<Uuid>,
}

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
    subcall_provider: Option<Arc<dyn Provider>>,
    subcall_model: Option<String>,
    observability: SummaryObservability,
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

    info!(
        start = range.start,
        end = range.end,
        target_tokens,
        "Producing summary via RLM"
    );

    let params = SummaryProduceParams {
        range,
        target_tokens,
        session_id,
        provider,
        model,
        subcall_provider,
        subcall_model: subcall_model.as_deref(),
        bus: observability.bus,
        trace_id: observability.trace_id,
    };
    let auto_ctx = build_auto_context(params);

    let result = crate::rlm::RlmRouter::auto_process(&input, auto_ctx, rlm_config)
        .await
        .context("RLM summarisation for SummaryIndex::summary_for failed")?;

    Ok(SummaryNode {
        content: super::summary_text::bounded_summary(result, target_tokens)?,
        target_tokens,
        granularity,
        generation,
    })
}
