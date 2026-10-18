//! Bind managed spawn commands to native session/workspace context.
use crate::{session::Session, tui::app::state::App};
use std::path::Path;
#[path = "spawn_agent_host.rs"]
mod host;
#[path = "spawn_agent_model.rs"]
pub mod model;
#[path = "spawn_agent_watch.rs"]
mod watch;
/// Spawn and immediately dispatch a managed child agent.
pub async fn handle_spawn_command(app: &mut App, session: &Session, cwd: &Path, rest: &str) {
    codetether_agent_command::spawn::handle(&mut host::Native { app, session, cwd }, rest).await;
}
