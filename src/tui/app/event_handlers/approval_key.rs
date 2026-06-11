//! Approval keyboard shortcuts for pending tool requests.

use crate::tui::app::input::approval_command;
use crate::tui::app::state::{App, approval_queue};

pub(super) fn approve(app: &mut App) -> bool {
    decide(app, "/approve")
}

pub(super) fn deny(app: &mut App) -> bool {
    decide(app, "/deny")
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
