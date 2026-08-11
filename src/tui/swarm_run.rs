//! Launch a real parallel swarm from the TUI `/swarm <task>` command.
//!
//! Bare `/swarm` opens the monitor view; `/swarm <task>` decomposes the task
//! and executes sub-agents in parallel, streaming [`SwarmEvent`]s to the view.

use crate::session::Session;
use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

#[path = "swarm_run/control.rs"]
mod control;
#[path = "swarm_run/monitor.rs"]
mod monitor;

use control::{ControlAction, apply_control};
pub use monitor::open_swarm_monitor;

/// Handle `/swarm [task]`.
///
/// Returns `true` when a task was launched; `false` for the bare command so
/// the caller falls through to opening the monitor view.
pub fn handle_swarm_command(app: &mut App, session: &Session, rest: &str) -> bool {
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

    let model = session.metadata.model.clone();
    let (tx, rx) = tokio::sync::mpsc::channel(256);
    app.state.swarm.attach_event_rx(rx);
    app.state.set_view_mode(ViewMode::Swarm);
    app.state.status = format!("Swarm running: {task}");
    app.state
        .messages
        .push(crate::tui::chat::message::ChatMessage::new(
            crate::tui::chat::message::MessageType::System,
            format!("Launching swarm for task: {task}"),
        ));
    app.state.scroll_to_bottom();

    let control = spawn_swarm_run(task.to_string(), model, tx);
    app.state.swarm.control = Some(control);
    true
}

#[path = "swarm_run/spawn.rs"]
mod spawn;
use spawn::spawn_swarm_run;
