//! Approval keyboard shortcuts for pending tool requests.

#[path = "approval_copy_key.rs"]
mod copy_key;
#[path = "approval_edit_key.rs"]
mod edit_key;
#[path = "approval_scroll_key.rs"]
mod scroll_key;
#[path = "approval_scroll_wheel.rs"]
mod scroll_wheel;

#[path = "approval_decision_key.rs"]
mod decision_key;
use crate::tui::app::state::App;
use decision_key::decide;

pub(super) use copy_key::copy_preview;
pub(super) use edit_key::open as edit;
pub(super) use scroll_key::handle as scroll;
pub(super) use scroll_wheel::handle as wheel;

#[cfg(test)]
#[path = "approval_navigation_key_tests.rs"]
mod navigation_tests;
#[cfg(test)]
#[path = "approval_scroll_limit_tests.rs"]
mod scroll_limit_tests;

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
