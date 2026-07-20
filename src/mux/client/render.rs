//! Text client rendering for network mux state.

use crate::mux::model::MuxSnapshot;
use crate::mux::protocol::ServerResponse;

pub(super) fn response(response: &ServerResponse) -> bool {
    match response {
        ServerResponse::Snapshot { state } => snapshot(state),
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

pub(super) fn snapshot(state: &MuxSnapshot) {
    println!("[{}]", state.name);
    for window in &state.windows {
        let active = if window.id == state.active_window {
            '*'
        } else {
            ' '
        };
        println!(
            " {active} {}:{}  {}",
            window.id,
            window.title,
            window.workspace.display()
        );
    }
}

pub(super) fn help() {
    println!("mux: ls | new PATH | cd PATH | select ID | close ID | attach | detach | kill | help");
    println!("programs: enter any other command, e.g. codetether tui --access-mode full");
    println!("folders: press Tab after cd or new to complete from the active workspace");
    println!("detach to your launching shell: Ctrl+B, then D");
}
