//! Native goal-editor projection into shared viewport rendering.
use crate::tui::app::state::App;
use ratatui::Frame;
pub(super) fn render(frame: &mut Frame, app: &mut App) {
    codetether_editor::viewport::draw_goal(frame, &mut super::editor::draw::view(&mut app.state));
}

#[cfg(test)]
#[path = "goal_editor_tests.rs"]
mod tests;
