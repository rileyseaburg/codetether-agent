//! CLI lifecycle operations for mux servers.

mod attach;
mod kill;
mod list;
mod new_session;
mod serve;
pub(in crate::mux) mod spawn;
pub(in crate::mux) mod startup;

use anyhow::Result;

use crate::cli::command::mux_args::MuxCommand;

pub(super) async fn execute(command: MuxCommand) -> Result<()> {
    match command {
        MuxCommand::New {
            session,
            directory,
            detached,
        } => new_session::run(session, directory, detached).await,
        MuxCommand::Attach { target } => attach::run(&target).await,
        MuxCommand::List { json } => list::run(json).await,
        MuxCommand::Kill { target } => kill::run(&target).await,
        MuxCommand::Serve {
            session,
            directory,
            bind,
        } => serve::run(session, directory, bind).await,
    }
}
