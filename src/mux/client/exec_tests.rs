use crate::mux::client::parse::{ParsedCommand, parse};

#[test]
fn program_command_is_preserved_for_server_startup() {
    let ParsedCommand::Exec(command) = parse("codetether tui --access-mode full") else {
        panic!("expected server-owned program command");
    };
    assert_eq!(command, "codetether tui --access-mode full");
}
