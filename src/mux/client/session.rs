//! Mux control prompt surrounding persistent PTY attachment.

mod dispatch;
mod helpers;
mod prompt;

use anyhow::Result;

use super::connection::MuxConnection;
use crate::mux::model::MuxSnapshot;

pub(super) async fn run(
    connection: &mut MuxConnection,
    mut state: Option<MuxSnapshot>,
) -> Result<()> {
    loop {
        let workspace = super::state::active_workspace(&state)?;
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
