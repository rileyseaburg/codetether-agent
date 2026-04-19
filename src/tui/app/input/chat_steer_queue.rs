//! Queue chat input while a request is in flight.
//!
//! Lets the user keep typing during streaming — the message is
//! appended to [`App::queued_steering`] and is automatically
//! submitted as a new user turn by
//! [`crate::tui::app::event_loop::auto_drain::auto_drain_queued_input`]
//! as soon as the current request completes.

use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

/// Append the current input to the queue and echo a system message
/// so the user sees it was accepted and will be auto-sent.
pub(super) fn queue_steering_while_processing(app: &mut App, prompt: &str) {
    if prompt.is_empty() {
        app.state.status =
            "Still processing — type a message and it will be sent when the turn finishes."
                .to_string();
        return;
    }

    app.state.queue_steering(prompt);
    let count = app.state.steering_count();
    app.state.messages.push(ChatMessage::new(
        MessageType::System,
        format!("Queued ({count}) — will send automatically when the current turn finishes: {prompt}"),
    ));
    app.state.clear_input();
    app.state.scroll_to_bottom();
    app.state.status = format!("Queued ({count}) — will auto-send after current turn");

    // Interrupt the in-flight provider turn so the queued message is
    // drained by `auto_drain_queued_input` immediately. Without this the
    // message sits in the queue until the LLM finishes on its own, which
    // defeats the point of steering mid-stream.
    //
    // Use `notify_one` (NOT `notify_waiters`): `notify_waiters` only
    // wakes already-parked waiters and silently drops the signal if
    // the spawned provider task hasn't yet polled `cancel.notified()`.
    // That race (Enter pressed before the task is scheduled) is what
    // made steering appear to do nothing. `notify_one` stores a permit
    // so the next `notified().await` returns immediately.
    //
    // Keep the Arc in place (don't `take`) so a second steering during
    // the same turn can also fire if needed.
    if let Some(cancel) = app.state.current_turn_cancel.as_ref() {
        cancel.notify_one();
        app.state.status = format!("Interrupting turn to apply {count} steering message(s)…");
    }
}
