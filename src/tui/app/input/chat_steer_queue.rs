//! Queue chat input as steering while a request is in flight.
//!
//! Allows the user to keep typing during streaming — the
//! message is appended to [`App::queued_steering`] and surfaces
//! on the next submitted prompt via
//! [`crate::tui::app::state::App::steering_prompt_prefix`].

use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

/// Append the current input to the steering queue and echo a
/// system message so the user sees it was accepted.
pub(super) fn queue_steering_while_processing(app: &mut App, prompt: &str) {
    if prompt.is_empty() {
        app.state.status = "Still processing — type a message to queue steering.".to_string();
        return;
    }

    app.state.queue_steering(prompt);
    let count = app.state.steering_count();
    app.state.messages.push(ChatMessage::new(
        MessageType::System,
        format!("Queued steering ({count}) for next turn: {prompt}"),
    ));
    app.state.clear_input();
    app.state.scroll_to_bottom();
    app.state.status = format!("Queued steering ({count}) — applied on next turn");
}
