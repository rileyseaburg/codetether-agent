//! Workspace validation and startup for one session-owned PTY program.

use crate::mux::protocol::ServerResponse;
use crate::mux::pty::TerminalSize;

use super::context::ServerContext;

pub(super) async fn start(
    context: &ServerContext,
    session: &str,
    id: u64,
    command: &str,
    size: TerminalSize,
) -> anyhow::Result<ServerResponse> {
    let workspace = {
        let state = context.state.read().await;
        state
            .session(session)
            .and_then(|item| item.window(id))
            .map(|window| window.workspace.clone())
            .ok_or_else(|| anyhow::anyhow!("window {id} does not exist"))?
    };
    let offset = context
        .programs
        .start(id, command, &workspace, size, session)?;
    Ok(ServerResponse::ProgramAttached {
        window_id: id,
        offset,
        replay_until: offset,
        alternate_screen: false,
    })
}
