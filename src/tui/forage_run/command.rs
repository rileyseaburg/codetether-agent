//! Launch a forage scan/execution run from the TUI `/forage` command.
//!
//! `/forage` scans active OKRs for opportunities; `/forage execute` also runs
//! the top selection with the build agent. Results stream back into chat.

use crate::session::Session;
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

use super::args::build_tui_forage_args;
use super::spawn::spawn_forage_run;

/// Handle `/forage [execute] [N]`.
///
/// Returns `true` once a run is launched. The optional `execute` keyword
/// enables build-agent execution; a trailing integer overrides `--top`.
pub fn handle_forage_command(app: &mut App, session: &Session, rest: &str) -> bool {
    let (execute, top) = parse_forage_args(rest);

    let model = session.metadata.model.clone();
    let (tx, rx) = tokio::sync::mpsc::channel(64);
    app.state.forage.attach_rx(rx);

    let mode = if execute { "scan + execute" } else { "scan" };
    app.state.status = format!("Forage running: {mode} (top {top})");
    app.state.messages.push(ChatMessage::new(
        MessageType::System,
        format!("Launching forage ({mode}, top {top})…"),
    ));
    app.state.scroll_to_bottom();

    spawn_forage_run(build_tui_forage_args(top, execute, model), tx);
    true
}

/// Parse `/forage` arguments into `(execute, top)`.
fn parse_forage_args(rest: &str) -> (bool, usize) {
    let mut execute = false;
    let mut top = 3usize;
    for tok in rest.split_whitespace() {
        match tok {
            "execute" | "--execute" | "-x" => execute = true,
            other => {
                if let Ok(n) = other.parse::<usize>() {
                    top = n;
                }
            }
        }
    }
    (execute, top)
}

#[cfg(test)]
mod tests {
    use super::parse_forage_args;

    #[test]
    fn defaults_to_scan_top_three() {
        assert_eq!(parse_forage_args(""), (false, 3));
    }

    #[test]
    fn parses_execute_and_top() {
        assert_eq!(parse_forage_args("execute 7"), (true, 7));
        assert_eq!(parse_forage_args("5 -x"), (true, 5));
    }
}
