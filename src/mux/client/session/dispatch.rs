//! Native clipboard adapter for shared command execution.
use super::super::state::ClientState;
use super::super::{connection::MuxConnection, parse::ParsedCommand};
use anyhow::Result;
pub(super) async fn execute(
    connection: &mut MuxConnection,
    state: &mut ClientState,
    command: ParsedCommand,
) -> Result<bool> {
    codetether_mux_attachment::execute_command(
        connection,
        state,
        command,
        super::super::proxy::clipboard_content,
    )
    .await
}
