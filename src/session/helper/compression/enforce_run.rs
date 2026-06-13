//! Cascade-then-truncate driver for context-window enforcement.

use std::sync::Arc;

use crate::provider::{Message, ToolDefinition};
use crate::session::SessionEvent;

use super::context::CompressContext;
use super::enforce_attempt::CascadeEnv;
use super::enforce_budget::BudgetCheck;
use super::enforce_cascade::run_cascade;
use super::enforce_run_tail::terminal_when_needed;

/// Run the keep-last cascade; when it cannot satisfy the budget, apply
/// terminal truncation as the last resort.
pub(super) async fn cascade_or_truncate(
    messages: &mut Vec<Message>,
    ctx: &CompressContext,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&tokio::sync::mpsc::Sender<SessionEvent>>,
    trace_id: uuid::Uuid,
    trigger_reason: &str,
    check: &BudgetCheck,
) -> anyhow::Result<()> {
    let env = CascadeEnv {
        trace_id,
        trigger_reason,
        initial_est: check.initial_est,
        safety_budget: check.safety_budget,
        ctx_window: check.ctx_window,
    };
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
