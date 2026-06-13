//! Terminal hand-off helper for `enforce_run`.

use tokio::sync::mpsc;
use uuid::Uuid;

use crate::provider::{Message, ToolDefinition};
use crate::session::SessionEvent;

use super::enforce_budget::BudgetCheck;
use super::enforce_terminal::apply_terminal;

/// Apply terminal truncation when the cascade did not fit the request;
/// otherwise no-op.
pub(super) async fn terminal_when_needed(
    fits: bool,
    messages: &mut Vec<Message>,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
    trace_id: Uuid,
    check: &BudgetCheck,
) {
    if fits {
        return;
    }
    apply_terminal(
        messages,
        system_prompt,
        tools,
        event_tx,
        trace_id,
        check.initial_est,
        check.safety_budget,
    )
    .await;
}
