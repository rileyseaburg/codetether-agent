//! Application bindings for interactive swarm commands and monitor guidance.
use crate::{session::Session, tui::app::state::App};
#[path = "swarm_run/host.rs"]
mod host;
#[cfg(test)]
#[path = "swarm_run/launch.rs"]
mod launch;
#[path = "swarm_run/monitor.rs"]
mod monitor;
pub use monitor::open_swarm_monitor;
/// Handle a swarm task or control command using the session's model.
pub fn handle_swarm_command(app: &mut App, session: &Session, rest: &str) -> bool {
    codetether_swarm_view::interactive::handle_swarm_command::<host::Runtime>(
        app,
        session.metadata.model.clone(),
        rest,
    )
}
