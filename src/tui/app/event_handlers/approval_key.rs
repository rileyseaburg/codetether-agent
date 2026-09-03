//! Approval keyboard shortcuts for pending tool requests.

#[path = "approval_copy_key.rs"]
mod copy_key;
#[path = "approval_edit_key.rs"]
mod edit_key;
#[path = "approval_scroll_key.rs"]
mod scroll_key;

use crate::tui::app::input::approval_command;
use crate::tui::app::state::{App, approval_queue};

pub(super) use copy_key::copy_preview;
pub(super) use edit_key::open as edit;
pub(super) use scroll_key::handle as scroll;

#[cfg(test)]
#[path = "approval_feedback_key_tests.rs"]
mod feedback_tests;
#[cfg(test)]
#[path = "approval_hidden_key_tests.rs"]
mod hidden_tests;
#[cfg(test)]
#[path = "approval_scroll_key_tests.rs"]
mod scroll_tests;

pub(super) fn handle(app: &mut App, character: char, cwd: &std::path::Path) -> bool {
    match character {
        'a' => decide(app, "/approve"),
        'd' => decide(app, "/deny"),
        'e' => edit(app, cwd),
        'y' => copy_preview(app),
        _ => false,
    }
}

fn decide(app: &mut App, command: &str) -> bool {
    if !pending() {
        return false;
    }
    approval_command::run(app, command)
}

fn pending() -> bool {
    approval_queue::active_id()
        .or_else(crate::approval::live::latest_id)
        .is_some()
}
