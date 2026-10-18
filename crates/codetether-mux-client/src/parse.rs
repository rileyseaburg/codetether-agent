//! Parse mux control commands while preserving arbitrary program command lines.
use crate::ParsedCommand;
use crate::parse_args::{with_id, with_workspace};
use codetether_mux_protocol::ClientRequest;

/// Parse a control command or preserve it as an arbitrary program invocation.
/// # Arguments
/// `line` is user-entered text; surrounding whitespace is trimmed.
/// # Returns
/// A typed command, including an Invalid result for missing paths or bad IDs.
/// # Examples
/// ```
/// assert!(matches!(codetether_mux_client::parse("help"), codetether_mux_client::ParsedCommand::Help));
/// ```
pub fn parse(line: &str) -> ParsedCommand {
    let (command, argument) = line.trim().split_once(' ').unwrap_or((line.trim(), ""));
    match command {
        "ls" => ParsedCommand::Request(ClientRequest::Snapshot),
        "new" => with_workspace(argument, |workspace| ClientRequest::CreateWindow {
            workspace,
        }),
        "cd" => with_workspace(argument, |workspace| ClientRequest::ChangeDirectory {
            workspace,
        }),
        "select" => with_id(argument, |id| ClientRequest::SelectWindow { id }),
        "close" => with_id(argument, |id| ClientRequest::CloseWindow { id }),
        "detach" | "quit" => ParsedCommand::Request(ClientRequest::Detach),
        "kill" => ParsedCommand::Kill,
        "help" | "?" => ParsedCommand::Help,
        "attach" => ParsedCommand::Attach,
        "" => ParsedCommand::Request(ClientRequest::Snapshot),
        _ => ParsedCommand::Exec(line.trim().into()),
    }
}
