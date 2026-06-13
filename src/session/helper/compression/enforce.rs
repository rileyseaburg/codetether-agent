//! Adaptive context-window enforcement: budget gate + cascade entry.

use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::provider::ToolDefinition;
use crate::session::SessionEvent;

use super::context::CompressContext;
use super::enforce_budget::check_budget;
use super::enforce_events::emit_started;
use super::enforce_run::cascade_or_truncate;

/// Ensure the estimated request token cost of `messages` fits within the
/// model's safety budget, in-place.
///
/// This is the `&mut Vec<Message>` core that powers both the legacy
/// [`enforce_context_window`](super::enforce_context_window) wrapper and
/// the Phase B [`derive_context`](crate::session::context::derive_context)
/// pipeline (which runs on a history clone).
///
/// # Errors
///
/// Returns `Err` only if an underlying RLM pass errors in a way the
/// recovery cascade cannot absorb. Terminal truncation is infallible.
pub(crate) async fn enforce_on_messages(
    messages: &mut Vec<crate::provider::Message>,
    ctx: &CompressContext,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
) -> Result<()> {
    let check = check_budget(messages, ctx, model, system_prompt, tools);
    let Some(trigger_reason) = check.trigger_reason else {
        return Ok(());
    };

    let trace_id = Uuid::new_v4();
    emit_started(
        event_tx,
        trace_id,
        trigger_reason,
        check.initial_est,
        check.safety_budget,
    )
    .await;

    cascade_or_truncate(
        messages,
        ctx,
        provider,
        model,
        system_prompt,
        tools,
        event_tx,
        trace_id,
        trigger_reason,
        &check,
    )
    .await
}
