//! Cascade-then-truncate driver for context-window enforcement.

use std::sync::Arc;

use codetether_provider_core::{Message, ToolDefinition};
use codetether_session_events::SessionEvent;

use super::enforce_attempt::CascadeEnv;
use super::enforce_budget::BudgetCheck;
use super::enforce_cascade::run_cascade;
use super::enforce_run_tail::terminal_when_needed;
use codetether_context_compression::CompressContext;

/// Run the keep-last cascade; when it cannot satisfy the budget, apply
/// terminal truncation as the last resort.
pub(super) async fn cascade_or_truncate<S>(
    messages: &mut Vec<Message>,
    ctx: &CompressContext,
    provider: Arc<dyn codetether_provider_core::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&tokio::sync::mpsc::Sender<SessionEvent<S>>>,
    trace_id: uuid::Uuid,
    trigger_reason: &str,
    check: &BudgetCheck,
) -> anyhow::Result<()> {
    let env = CascadeEnv::from_check(trace_id, trigger_reason, check);
    let fits = run_cascade(
        messages,
        ctx,
        provider,
        model,
        system_prompt,
        tools,
        event_tx,
        &env,
    )
    .await?;
    terminal_when_needed(
        fits,
        messages,
        system_prompt,
        tools,
        event_tx,
        trace_id,
        check,
    )
    .await;
    Ok(())
}
