//! Application status and redraw updates after goal-prompt input.
use crate::tui::app::state::App;
use codetether_goal_prompt::KeyOutcome;
use crossterm::event::KeyEvent;
/// Apply a goal-prompt key and report whether the modal consumed it.
pub(super) fn handle_goal_prompt_key(app: &mut App, key: KeyEvent) -> bool {
    match codetether_goal_prompt::handle_key(&mut app.state.goal_prompt, key) {
        KeyOutcome::Inactive => return false,
        KeyOutcome::Updated => {}
        KeyOutcome::Finished => app.state.status = "Goals saved — getting to work.".to_string(),
    }
    app.state.needs_redraw = true;
    true
}
