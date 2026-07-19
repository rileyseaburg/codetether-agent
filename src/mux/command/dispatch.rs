//! Mux lifecycle command routing.

use anyhow::Result;

use crate::cli::command::mux_args::MuxCommand;

pub(in crate::mux) async fn execute(command: MuxCommand) -> Result<()> {
    match command {
        MuxCommand::New {
            session,
            directory,
            detached,
        } => super::new_session::run(session, directory, detached).await,
        MuxCommand::Attach { target } => super::attach::run(&target).await,
        MuxCommand::List { json } => super::list::run(json).await,
        MuxCommand::Kill { target } => super::kill::run(&target).await,
        MuxCommand::KillAll => super::kill_all::run().await,
        MuxCommand::Serve {
            session,
            directory,
            bind,
        } => super::serve::run(session, directory, bind).await,
    }
}
