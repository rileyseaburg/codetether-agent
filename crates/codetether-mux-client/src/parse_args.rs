//! Shared argument validation for interactive mux commands.
use crate::ParsedCommand;
use codetether_mux_protocol::ClientRequest;
use std::path::PathBuf;

pub(super) fn with_workspace(
    argument: &str,
    request: impl FnOnce(PathBuf) -> ClientRequest,
) -> ParsedCommand {
    if argument.trim().is_empty() {
        return ParsedCommand::Invalid("workspace path is required".into());
    }
    ParsedCommand::Request(request(PathBuf::from(argument.trim())))
}

pub(super) fn with_id(argument: &str, request: impl FnOnce(u64) -> ClientRequest) -> ParsedCommand {
    match argument.trim().parse() {
        Ok(id) => ParsedCommand::Request(request(id)),
        Err(_) => ParsedCommand::Invalid("numeric window id is required".into()),
    }
}
