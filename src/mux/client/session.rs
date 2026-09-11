//! Mux control prompt surrounding persistent PTY attachment.

mod dispatch;
mod helpers;
mod prompt;

use anyhow::Result;

use super::connection::MuxConnection;
use super::state::ClientState;

pub(super) async fn run(connection: &mut MuxConnection, mut state: ClientState) -> Result<()> {
    loop {
        let workspace = state.active_workspace()?;
        let Some(line) = prompt::read(workspace).await? else {
            break;
        };
        let command = super::parse::parse(&line);
        let detached = dispatch::execute(connection, &mut state, command).await?;
        if detached {
            return Ok(());
        }
    }
    helpers::detach(connection).await;
    Ok(())
}
