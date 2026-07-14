use crate::mux::client::parse::{ParsedCommand, parse};
use crate::mux::protocol::ClientRequest;

#[test]
fn new_command_preserves_workspace_with_spaces() {
    let ParsedCommand::Request(ClientRequest::CreateWindow { workspace }) =
        parse("new /tmp/my project")
    else {
        panic!("expected create-window request");
    };
    assert_eq!(workspace, std::path::PathBuf::from("/tmp/my project"));
}

#[test]
fn close_command_targets_requested_window() {
    let ParsedCommand::Request(ClientRequest::CloseWindow { id }) = parse("close 7") else {
        panic!("expected close-window request");
    };
    assert_eq!(id, 7);
}

#[test]
fn arbitrary_program_is_executed_in_active_window() {
    let ParsedCommand::Exec(command) = parse("codetether tui --access-mode full") else {
        panic!("expected foreground program command");
    };
    assert_eq!(command, "codetether tui --access-mode full");
}
