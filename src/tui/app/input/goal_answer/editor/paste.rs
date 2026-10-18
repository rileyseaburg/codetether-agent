//! Native buffer binding for shared goal-draft paste handling.
use crate::tui::app::state::App;
pub(crate) fn paste(app: &mut App, text: &str) -> bool {
    codetether_goal_editor::paste(&mut super::host::Editor { app }, text)
}
