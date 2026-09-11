//! Snapshot listing for the attached session.

use crate::mux::model::MuxSnapshot;

/// Print the attached session's windows; sibling sessions are listed by name only.
pub(super) fn print(state: &MuxSnapshot, session: &str) {
    println!("[{session}] {}", state.workspace.display());
    let Some(current) = state.session(session) else {
        return;
    };
    for window in &current.windows {
        let active = if window.id == current.active_window {
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
    let others: Vec<_> = state
        .sessions
        .iter()
        .filter(|item| item.name != session)
        .map(|item| item.name.as_str())
        .collect();
    if !others.is_empty() {
        println!(" other sessions on this workspace: {}", others.join(", "));
    }
}
