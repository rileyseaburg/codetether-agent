//! Cancel / pause / resume handling for a running TUI swarm.

use super::View;

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
pub(super) fn apply_control(app: &mut impl View, action: ControlAction) -> bool {
    let Some(control) = app.swarm().control.as_ref() else {
        app.set_status("No swarm is running".to_string());
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
    app.set_status(msg.to_string());
    app.push_system(msg.to_string());
    app.scroll_to_bottom();
    true
}
