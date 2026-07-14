//! Execution of one parsed mux control command.

use anyhow::Result;

use crate::mux::model::MuxSnapshot;

use super::super::{connection::MuxConnection, parse::ParsedCommand};

pub(super) async fn execute(
    connection: &mut MuxConnection,
    state: &mut Option<MuxSnapshot>,
    command: ParsedCommand,
) -> Result<bool> {
    let detached = match command {
        ParsedCommand::Help => {
            super::super::render::help();
            false
        }
        ParsedCommand::Invalid(message) => {
            eprintln!("mux: {message}");
            false
        }
        ParsedCommand::Exec(command) => {
            let id = super::super::state::active_id(state)?;
            let outcome = super::super::program::start(connection, id, command).await?;
            super::helpers::finish_program(connection, outcome).await
        }
        ParsedCommand::Attach => {
            let id = super::super::state::active_id(state)?;
            match super::super::program::attach(connection, id).await? {
                Some(outcome) => super::helpers::finish_program(connection, outcome).await,
                None => {
                    eprintln!("mux: active window has no running program");
                    false
                }
            }
        }
        ParsedCommand::Request(request) => {
            super::helpers::control(connection, state, request).await?
        }
    };
    Ok(detached)
}
