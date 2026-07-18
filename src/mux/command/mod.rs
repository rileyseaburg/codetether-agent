//! CLI lifecycle operations for mux servers.

mod attach;
mod kill;
pub(in crate::mux) mod kill_all;
mod list;
mod new_session;
mod serve;
mod shutdown;
pub(in crate::mux) mod spawn;
pub(in crate::mux) mod startup;
mod terminate;
mod terminate_identity;

#[cfg(test)]
mod shutdown_tests;

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
        MuxCommand::Kill {
            target,
            named_target,
        } => {
            let target = target
                .or(named_target)
                .ok_or_else(|| anyhow::anyhow!("mux target is required"))?;
            kill::run(&target).await
        }
        MuxCommand::KillAll => kill_all::run().await,
        MuxCommand::Serve {
            session,
            directory,
            bind,
        } => serve::run(session, directory, bind).await,
    }
}
