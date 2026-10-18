//! Text client rendering for network mux state.

use codetether_mux_protocol::ServerResponse;

/// Print user-facing response text and report whether the client should detach.
/// # Arguments
/// `response` is the server reply; `session` selects the visible snapshot window list.
/// # Returns
/// True for detached or shutting-down responses, false otherwise.
/// # Examples
/// ```
/// assert!(!codetether_mux_client::render::response(&codetether_mux_protocol::ServerResponse::Acknowledged, "work"));
/// ```
pub fn response(response: &ServerResponse, session: &str) -> bool {
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

/// Print the mux control and program-launch command help.
/// # Arguments
/// This function takes no arguments.
/// # Returns
/// Unit after writing the help lines to stdout.
/// # Examples
/// ```no_run
/// codetether_mux_client::render::help();
/// ```
pub fn help() {
    println!("mux: ls | new PATH | cd PATH | select ID | close ID | attach | detach | kill | help");
    println!("programs: enter any other command, e.g. codetether tui --access-mode full");
    println!("folders: press Tab after cd or new to complete from the active workspace");
    println!("kill closes this session only; the server exits with its last session");
    println!("detach to your launching shell: Ctrl+B, then D");
}
