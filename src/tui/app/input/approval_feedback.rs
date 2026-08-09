//! Direct text steering for a pending approval request.

use crate::tui::app::state::{App, approval_queue};

/// Sends ordinary chat input as the reason for rejecting and revising a tool call.
pub(super) fn submit(app: &mut App) -> bool {
    super::image_sidecar_recover::recover_pasted_images(app);
    let prompt = app.state.input.trim().to_string();
    if prompt.is_empty() || prompt.starts_with('/') {
        return false;
    }
    let Some(id) = approval_queue::active_id() else {
        return false;
    };
    super::approval_command::run(app, &format!("/deny {id} {prompt}"))
}

#[cfg(test)]
#[path = "approval_feedback_tests.rs"]
mod tests;
