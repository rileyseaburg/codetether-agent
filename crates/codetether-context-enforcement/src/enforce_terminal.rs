//! Terminal truncation hand-off when the RLM cascade is exhausted.

use tokio::sync::mpsc;
use uuid::Uuid;

use codetether_provider_core::{Message, ToolDefinition};
use codetether_request_tokens::estimate_request_tokens;
use codetether_session_events::SessionEvent;

use super::enforce_terminal_events::emit_terminal_events;
use super::terminal::terminal_truncate_messages;

/// Number of most-recent messages retained by terminal truncation.
const TRUNCATE_KEEP_LAST: usize = 4;

/// Apply terminal truncation and emit the truncation/outcome events.
pub(super) async fn apply_terminal<S>(
    messages: &mut Vec<Message>,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&mpsc::Sender<SessionEvent<S>>>,
    trace_id: Uuid,
    initial_est: usize,
    safety_budget: usize,
) {
    let dropped_tokens = terminal_truncate_messages(
        messages,
        system_prompt,
        tools,
        TRUNCATE_KEEP_LAST,
        safety_budget,
    );
    let after_tokens = estimate_request_tokens(system_prompt, messages, tools);

    tracing::warn!(target: "codetether_agent::session::helper::compression::enforce_terminal",
        before_tokens = initial_est,
        after_tokens,
        dropped_tokens,
        kept_messages = messages.len(),
        safety_budget,
        "All RLM compaction attempts exhausted; applied terminal truncation"
    );

    emit_terminal_events(
        event_tx,
        trace_id,
        initial_est,
        after_tokens,
        dropped_tokens,
        messages.len(),
        safety_budget,
    )
    .await;
}
