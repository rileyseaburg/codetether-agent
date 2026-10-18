//! Parse swarm task and runtime-control commands.
use super::control::{ControlAction, apply_control};
use super::{Host, View, launch};

/// Handle `/swarm [task]`.
///
/// Returns `true` when a task was launched; `false` for the bare command so
/// the caller falls through to opening the monitor view. Control commands and
/// an already-active run also return `true`.
///
/// # Arguments
/// * `app` — View receiving state, status and chat updates.
/// * `model` — Optional model selection for a new run.
/// * `rest` — Text following `/swarm`.
///
/// # Returns
/// Whether the command was handled without opening the bare monitor.
///
/// # Examples
/// ```no_run
/// # fn command<H: codetether_swarm_view::interactive::Host>(
/// #     view: &mut impl codetether_swarm_view::interactive::View) {
/// let handled = codetether_swarm_view::interactive::handle_swarm_command::<H>(
///     view, None, "pause",
/// );
/// assert!(handled);
/// # }
/// ```
pub fn handle_swarm_command<H: Host>(
    app: &mut impl View,
    model: Option<String>,
    rest: &str,
) -> bool {
    let task = rest.trim();
    if task.is_empty() {
        return false;
    }

    // Control sub-commands act on the currently running swarm.
    match task {
        "cancel" | "stop" | "kill" => return apply_control(app, ControlAction::Cancel),
        "pause" => return apply_control(app, ControlAction::Pause),
        "resume" | "continue" => return apply_control(app, ControlAction::Resume),
        _ => {}
    }

    launch::start::<H>(app, model, task)
}
