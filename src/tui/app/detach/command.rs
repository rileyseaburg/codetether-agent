//! Native entry point for the shared detach workflow.
use crate::{session::Session, tui::app::state::App};
/// Persist and register a background snapshot of the current conversation.
pub async fn handle_detach_command(app: &mut App, session: &mut Session, rest: &str) {
    codetether_agent_command::detach::handle(app, session, rest).await;
}
