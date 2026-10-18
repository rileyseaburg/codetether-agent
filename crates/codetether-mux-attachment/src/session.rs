//! Mux control prompt surrounding persistent PTY attachment.

use anyhow::Result;

use codetether_mux_client::{ClientState, MuxConnection};

pub(super) async fn run(
    connection: &mut MuxConnection,
    mut state: ClientState,
    clipboard: impl Fn() -> Option<String>,
) -> Result<()> {
    loop {
        let workspace = state.active_workspace()?;
        let Some(line) = codetether_command_prompt::read(workspace).await? else {
            break;
        };
        let command = codetether_mux_client::parse(&line);
        let detached = crate::execute_command(connection, &mut state, command, &clipboard).await?;
        if detached {
            return Ok(());
        }
    }
    crate::detach(connection).await;
    Ok(())
}
