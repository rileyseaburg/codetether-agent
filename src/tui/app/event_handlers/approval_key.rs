//! Native approval action bindings and regression coverage.
#[path = "approval_action_host.rs"]
mod host;
use crate::tui::app::state::App;
use crossterm::event::{KeyCode, KeyEvent};
use std::path::Path;
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

pub(super) fn handle(app: &mut App, character: char, cwd: &Path) -> bool {
    codetether_approval_view::interactive::handle(&mut host::Native(app), character, cwd)
}
pub(super) fn copy_preview(app: &mut App) -> bool {
    codetether_approval_view::interactive::copy_preview(&mut host::Native(app))
}
pub(super) fn edit(app: &mut App, cwd: &Path) -> bool {
    codetether_approval_view::interactive::open(&mut host::Native(app), cwd)
}
pub(super) fn scroll(app: &mut App, key: KeyEvent) -> bool {
    codetether_approval_view::interactive::scroll(&mut host::Native(app), key)
}
pub(super) fn wheel(app: &mut App, direction: KeyCode, amount: usize) -> bool {
    codetether_approval_view::interactive::wheel(&mut host::Native(app), direction, amount)
}
