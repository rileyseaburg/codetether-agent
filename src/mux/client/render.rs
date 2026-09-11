//! Text client rendering for network mux state.

use crate::mux::protocol::ServerResponse;

pub(super) fn response(response: &ServerResponse, session: &str) -> bool {
    match response {
        ServerResponse::Snapshot { state } => super::render_snapshot::print(state, session),
        ServerResponse::Error { message } => eprintln!("mux: {message}"),
        ServerResponse::Detached => {
            println!("detached");
            return true;
        }
        ServerResponse::ShuttingDown => {
            println!("mux server stopped");
            return true;
        }
        ServerResponse::Authenticated { .. } => {}
        ServerResponse::ProgramAttached { .. }
        | ServerResponse::ProgramOutput { .. }
        | ServerResponse::Coordination { .. }
        | ServerResponse::Agent { .. }
        | ServerResponse::Acknowledged => {}
    }
    false
}

pub(super) fn help() {
    println!("mux: ls | new PATH | cd PATH | select ID | close ID | attach | detach | kill | help");
    println!("programs: enter any other command, e.g. codetether tui --access-mode full");
    println!("folders: press Tab after cd or new to complete from the active workspace");
    println!("kill closes this session only; the server exits with its last session");
    println!("detach to your launching shell: Ctrl+B, then D");
}
