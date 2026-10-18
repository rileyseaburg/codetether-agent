//! Application status and redraw updates after goal-prompt input.
use crate::tui::app::state::App;
use crossterm::event::KeyEvent;
/// Apply a goal-prompt key and report whether the modal consumed it.
pub(super) fn handle_goal_prompt_key(app: &mut App, key: KeyEvent) -> bool {
    codetether_goal_prompt::handle_key_with_feedback(
        &mut app.state.goal_prompt,
        &mut app.state.status,
        &mut app.state.needs_redraw,
        key,
    )
}
