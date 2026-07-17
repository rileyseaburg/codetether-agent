//! Submit text and images to the active local prompt run.

use crate::session::helper::steering::SteeringInput;
use crate::tui::app::session_runtime::TuiSessionHandle;
use crate::tui::app::state::{App, prompt_queue};

use super::chat_helpers::push_user_messages;
use super::pasted_text::expand_paste_placeholders;

/// Inject pending text and images into the active run, or queue a fallback.
pub(super) fn submit(app: &mut App, runtime: &TuiSessionHandle, prompt: &str) {
    let images = app.state.pending_images.clone();
    if prompt.is_empty() && images.is_empty() {
        return;
    }
    let expanded = expand_paste_placeholders(prompt, &app.state.pending_text_pastes);
    let input = SteeringInput::new(expanded, images.clone());
    if !runtime.steer_current(input) {
        queue_fallback(app, prompt);
        return;
    }
    app.state.pending_images.clear();
    app.state.pending_text_pastes.clear();
    push_user_messages(app, prompt, &images);
    app.state.clear_input();
    app.state.status = "Steered current turn".to_string();
    app.state.scroll_to_bottom();
}

fn queue_fallback(app: &mut App, prompt: &str) {
    if prompt.is_empty() {
        app.state.status = "Image ready for the next turn".to_string();
    } else {
        prompt_queue::store(app, prompt.to_string());
    }
}
