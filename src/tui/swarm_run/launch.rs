//! Start a swarm after validating the current monitor state.

use super::spawn::spawn_swarm_run;
use crate::session::Session;
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::models::ViewMode;

pub(super) fn start(app: &mut App, session: &Session, task: &str) -> bool {
    if app.state.swarm.active {
        app.state.status = "A swarm is already running; cancel it before starting another.".into();
        return true;
    }

    let model = session.metadata.model.clone();
    let (tx, rx) = tokio::sync::mpsc::channel(256);
    app.state.swarm.attach_event_rx(rx);
    app.state.set_view_mode(ViewMode::Swarm);
    app.state.status = format!("Swarm running: {task}");
    app.state.messages.push(ChatMessage::new(
        MessageType::System,
        format!("Launching swarm for task: {task}"),
    ));
    app.state.scroll_to_bottom();

    app.state.swarm.control = Some(spawn_swarm_run(task.to_string(), model, tx));
    true
}

#[cfg(test)]
#[path = "launch_tests.rs"]
mod tests;
