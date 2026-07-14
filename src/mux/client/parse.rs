//! Interactive mux client command parsing.

use std::path::PathBuf;

use crate::mux::protocol::ClientRequest;

pub(in crate::mux) enum ParsedCommand {
    Request(ClientRequest),
    Exec(String),
    Attach,
    Help,
    Invalid(String),
}

pub(in crate::mux) fn parse(line: &str) -> ParsedCommand {
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
        "kill" => ParsedCommand::Request(ClientRequest::Shutdown),
        "help" | "?" => ParsedCommand::Help,
        "attach" => ParsedCommand::Attach,
        "" => ParsedCommand::Request(ClientRequest::Snapshot),
        _ => ParsedCommand::Exec(line.trim().into()),
    }
}

fn with_workspace(argument: &str, request: impl FnOnce(PathBuf) -> ClientRequest) -> ParsedCommand {
    if argument.trim().is_empty() {
        return ParsedCommand::Invalid("workspace path is required".into());
    }
    ParsedCommand::Request(request(PathBuf::from(argument.trim())))
}

fn with_id(argument: &str, request: impl FnOnce(u64) -> ClientRequest) -> ParsedCommand {
    match argument.trim().parse() {
        Ok(id) => ParsedCommand::Request(request(id)),
        Err(_) => ParsedCommand::Invalid("numeric window id is required".into()),
    }
}
