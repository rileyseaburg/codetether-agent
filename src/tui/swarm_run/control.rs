//! Cancel / pause / resume handling for a running TUI swarm.

use crate::tui::app::state::App;

/// Which control action to apply to the active swarm.
pub(super) enum ControlAction {
    Cancel,
    Pause,
    Resume,
}

/// Apply a control action to the currently running swarm, updating status.
///
/// Returns `true` so the slash-command dispatcher treats the command as
/// handled (it never falls through to launching a new swarm task).
pub(super) fn apply_control(app: &mut App, action: ControlAction) -> bool {
    let Some(control) = app.state.swarm.control.as_ref() else {
        app.state.status = "No swarm is running".to_string();
        return true;
    };
    let msg = match action {
        ControlAction::Cancel => {
            control.cancel();
            "Swarm cancellation requested"
        }
        ControlAction::Pause => {
            control.pause();
            "Swarm paused (current stage finishes, no new stages start)"
        }
        ControlAction::Resume => {
            control.resume();
            "Swarm resumed"
        }
    };
    app.state.status = msg.to_string();
    app.state
        .messages
        .push(crate::tui::chat::message::ChatMessage::new(
            crate::tui::chat::message::MessageType::System,
            msg.to_string(),
        ));
    app.state.scroll_to_bottom();
    true
}
