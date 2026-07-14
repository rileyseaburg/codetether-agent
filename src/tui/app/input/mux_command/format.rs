//! Human-readable mux state for TUI system messages.

use crate::mux::control::MuxSessionSummary;

pub(super) const HELP: &str = "Mux commands:\n/mux ls\n/mux new NAME [DIRECTORY]\n/mux window NAME [DIRECTORY]\n/mux select NAME WINDOW_ID\n/mux close NAME WINDOW_ID\n/mux kill NAME";

pub(super) fn sessions(items: Vec<MuxSessionSummary>) -> String {
    if items.is_empty() {
        return "No mux sessions.".into();
    }
    items.iter().map(session).collect::<Vec<_>>().join("\n")
}

pub(super) fn changed(action: &str, item: &MuxSessionSummary) -> String {
    format!("{action} mux session:\n{}", session(item))
}

fn session(item: &MuxSessionSummary) -> String {
    let status = if item.reachable { "up" } else { "stale" };
    let header = format!(
        "{}: {} windows (active {}) [{status}] pid={} {}",
        item.name,
        item.windows.len(),
        item.active_window,
        item.pid,
        item.address
    );
    let windows = item
        .windows
        .iter()
        .map(|window| {
            let marker = if window.id == item.active_window {
                "*"
            } else {
                " "
            };
            format!(
                "  {marker} {} {} — {}",
                window.id,
                window.title,
                window.workspace.display()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!("{header}\n{windows}")
}
