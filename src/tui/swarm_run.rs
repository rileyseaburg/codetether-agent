//! Launch a real parallel swarm from the TUI `/swarm <task>` command.
//!
//! Bare `/swarm` opens the monitor view; `/swarm <task>` decomposes the task
//! and executes sub-agents in parallel, streaming [`SwarmEvent`]s to the view.

use crate::session::Session;
use crate::swarm::{DecompositionStrategy, SwarmConfig, SwarmExecutor};
use crate::tui::app::state::App;
use crate::tui::models::ViewMode;
use crate::tui::swarm_view::SwarmEvent;

#[path = "swarm_run/monitor.rs"]
mod monitor;

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

    spawn_swarm_run(task.to_string(), model, tx);
    true
}

fn spawn_swarm_run(
    task: String,
    model: Option<String>,
    event_tx: tokio::sync::mpsc::Sender<SwarmEvent>,
) {
    tokio::spawn(async move {
        let executor = SwarmExecutor::new(SwarmConfig {
            model,
            ..Default::default()
        })
        .with_event_tx(event_tx.clone());

        if let Err(err) = executor
            .execute(&task, DecompositionStrategy::Automatic)
            .await
        {
            let _ = event_tx
                .send(SwarmEvent::Error(format!("Swarm execution failed: {err}")))
                .await;
        }
    });
}
