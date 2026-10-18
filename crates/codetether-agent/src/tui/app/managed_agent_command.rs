//! Native entry points for shared agent commands.
use crate::{session::Session, tui::app::state::App};
#[path = "managed_agent_admin.rs"]
pub(crate) mod admin;
#[path = "managed_agent_command_host/mod.rs"]
pub(super) mod host;
#[cfg(test)]
#[path = "managed_agent_swarm.rs"]
mod swarm;
pub(crate) async fn handle(app: &mut App, session: &Session, rest: &str) {
    codetether_agent_command::handle(
        &mut host::Native {
            app,
            parent: Some(&session.id),
        },
        rest,
    )
    .await;
}
