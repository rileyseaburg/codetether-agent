//! Workspace validation and startup for one server-owned PTY program.

use crate::mux::protocol::ServerResponse;
use crate::mux::pty::TerminalSize;

use super::context::ServerContext;

pub(super) async fn start(
    context: &ServerContext,
    id: u64,
    command: &str,
    size: TerminalSize,
) -> anyhow::Result<ServerResponse> {
    let state = context.state.read().await;
    let workspace = state
        .windows
        .iter()
        .find(|window| window.id == id)
        .ok_or_else(|| anyhow::anyhow!("window {id} does not exist"))?
        .workspace
        .clone();
    drop(state);
    let offset = context.programs.start(id, command, &workspace, size)?;
    Ok(ServerResponse::ProgramAttached {
        window_id: id,
        offset,
        alternate_screen: false,
    })
}
