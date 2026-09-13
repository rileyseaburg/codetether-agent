//! TUI side of [`crate::a2a::live_inbox`]: run a peer's turn here, reply there.
//!
//! One parked turn is admitted at a time, only when the session is idle.
//! Its responder is held in `AppState::live_inbound` until the turn
//! finishes; `settle` then sends the assistant's final text back to the
//! waiting `message/send` handler. Announcement and dispatch reuse the
//! existing worker-task path in `inbox.rs`.

use crate::a2a::live_inbox;
use crate::provider::Role;
use crate::tui::app::message_text::extract_message_text;
use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;
use crate::tui::worker_bridge::IncomingTask;

/// Move the next parked peer turn onto the worker-task queue, if idle.
pub(crate) fn admit(app: &mut App) {
    if app.state.processing || app.state.live_inbound.is_some() {
        return;
    }
    let Some(inbound) = live_inbox::dequeue() else {
        return;
    };
    app.state.enqueue_worker_task(IncomingTask {
        task_id: inbound.task_id.clone(),
        message: inbound.prompt.clone(),
        from_agent: Some(inbound.from.clone()),
    });
    app.state.live_inbound = Some(inbound);
}

/// Resolve the in-flight peer turn with the session's final assistant text.
pub(crate) fn settle(app: &mut App, outcome: Result<String, String>) {
    if let Some(inbound) = app.state.live_inbound.take() {
        tracing::info!(task_id = %inbound.task_id, ok = outcome.is_ok(), "Settled live A2A turn");
        inbound.responder.resolve(outcome);
    }
}

/// Settle with the most recent assistant message in `slot`.
pub(crate) fn settle_from_session(app: &mut App, slot: &SessionSlot) {
    if app.state.live_inbound.is_none() {
        return;
    }
    let text = slot
        .borrow()
        .and_then(|session| {
            session
                .messages
                .iter()
                .rev()
                .find(|message| message.role == Role::Assistant)
        })
        .map(|message| extract_message_text(&message.content))
        .unwrap_or_default();
    settle(app, Ok(text));
}
