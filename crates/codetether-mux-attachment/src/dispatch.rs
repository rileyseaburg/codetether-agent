//! Execute one parsed interactive mux command.
use anyhow::Result;
use codetether_mux_client::{ClientState, MuxConnection, ParsedCommand};

/// Execute one parsed control command against the selected session/window.
/// # Arguments
/// Connection, snapshot state, parsed command and local clipboard callback.
/// # Returns
/// True when the client should detach or the session was closed.
/// # Errors
/// Propagates unavailable-window, transport, terminal and server failures.
/// # Examples
/// ```no_run
/// # async fn example(connection: &mut codetether_mux_client::MuxConnection) -> anyhow::Result<()> {
/// let mut state = codetether_mux_client::ClientState::new("work");
/// let _done = codetether_mux_attachment::execute_command(connection, &mut state, codetether_mux_client::ParsedCommand::Help, || None).await?;
/// # Ok(()) }
/// ```
pub async fn execute(
    connection: &mut MuxConnection,
    state: &mut ClientState,
    command: ParsedCommand,
    clipboard: impl Fn() -> Option<String>,
) -> Result<bool> {
    let detached = match command {
        ParsedCommand::Help => {
            codetether_mux_client::render::help();
            false
        }
        ParsedCommand::Invalid(message) => {
            eprintln!("mux: {message}");
            false
        }
        ParsedCommand::Exec(command) => {
            crate::program_dispatch::start(connection, state, command, clipboard).await?
        }
        ParsedCommand::Attach => {
            crate::program_dispatch::attach(connection, state, clipboard).await?
        }
        ParsedCommand::Kill => {
            let request = codetether_mux_protocol::ClientRequest::CloseSession {
                name: state.session.clone(),
            };
            crate::control::control(connection, state, request).await? || {
                println!("closed mux session '{}'", state.session);
                true
            }
        }
        ParsedCommand::Request(request) => {
            crate::control::control(connection, state, request).await?
        }
    };
    Ok(detached)
}
