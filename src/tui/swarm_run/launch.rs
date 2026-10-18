//! Compatibility launch entry point for application integration tests.
use crate::{session::Session, tui::app::state::App};
#[cfg(test)]
pub(super) fn start(app: &mut App, session: &Session, task: &str) -> bool {
    super::handle_swarm_command(app, session, task)
}
#[cfg(test)]
#[path = "launch_tests.rs"]
mod tests;
