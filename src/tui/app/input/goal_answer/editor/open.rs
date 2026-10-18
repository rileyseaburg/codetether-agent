//! Native editor view bindings for shared draft opening and discard.
use crate::tui::app::state::App;
pub(crate) fn open(app: &mut App, id: &str) {
    codetether_goal_editor::open(&mut super::host::Editor { app }, id);
}
pub(crate) fn close(app: &mut App) {
    codetether_goal_editor::close(&mut super::host::Editor { app });
}
