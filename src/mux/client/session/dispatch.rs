//! Execution of one parsed mux control command.

use anyhow::Result;

use super::super::state::ClientState;
use super::super::{connection::MuxConnection, parse::ParsedCommand};

pub(super) async fn execute(
    connection: &mut MuxConnection,
    state: &mut ClientState,
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
            let id = state.active_id()?;
            let outcome = super::super::program::start(connection, id, command).await?;
            super::helpers::finish_program(connection, outcome).await
        }
        ParsedCommand::Attach => {
            let id = state.active_id()?;
            match super::super::program::attach(connection, id).await? {
                Some(outcome) => super::helpers::finish_program(connection, outcome).await,
                None => {
                    eprintln!("mux: active window has no running program");
                    false
                }
            }
        }
        ParsedCommand::Kill => {
            let request = crate::mux::protocol::ClientRequest::CloseSession {
                name: state.session.clone(),
            };
            super::helpers::control(connection, state, request).await? || {
                println!("closed mux session '{}'", state.session);
                true
            }
        }
        ParsedCommand::Request(request) => {
            super::helpers::control(connection, state, request).await?
        }
    };
    Ok(detached)
}
