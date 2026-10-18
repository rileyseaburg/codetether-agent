//! Admit one parked turn and settle its response on completion.
use crate::Inbox;
use codetether_a2a_inbox::Inbound;
use codetether_chat_transcript::extract_message_text;
use codetether_provider_core::{Message, Role};
use codetether_worker_events::IncomingTask;
/// Move the next parked peer turn onto the worker-task queue, if idle.
/// # Arguments
/// Receives the frontend bindings or active inbound slot and the operation's input.
/// # Returns
/// Applies the corresponding A2A transition while retaining the existing queue order.
pub fn admit(view: &mut impl Inbox) {
    if view.processing() || view.inbound().is_some() {
        return;
    }
    let Some(inbound) = codetether_a2a_inbox::dequeue() else {
        return;
    };
    view.enqueue_task(IncomingTask {
        task_id: inbound.task_id.clone(),
        message: inbound.prompt.clone(),
        from_agent: Some(inbound.from.clone()),
    });
    *view.inbound() = Some(inbound);
}

/// Resolve the in-flight peer turn with the session's final assistant text.
/// # Arguments
/// Receives the frontend bindings or active inbound slot and the operation's input.
/// # Returns
/// Applies the corresponding A2A transition while retaining the existing queue order.
pub fn settle(active: &mut Option<Inbound>, outcome: Result<String, String>) {
    if let Some(inbound) = active.take() {
        tracing::info!(target: "codetether_agent::tui::app::inbox::live", task_id = %inbound.task_id, ok = outcome.is_ok(), "Settled live A2A turn");
        inbound.responder.resolve(outcome);
    }
}

/// Settle with the most recent assistant message in `slot`.
/// # Arguments
/// Receives the frontend bindings or active inbound slot and the operation's input.
/// # Returns
/// Applies the corresponding A2A transition while retaining the existing queue order.
pub fn settle_from_session(active: &mut Option<Inbound>, messages: Option<&[Message]>) {
    if active.is_none() {
        return;
    }
    let text = messages
        .and_then(|messages| {
            messages
                .iter()
                .rev()
                .find(|message| message.role == Role::Assistant)
        })
        .map(|message| extract_message_text(&message.content))
        .unwrap_or_default();
    settle(active, Ok(text));
}
