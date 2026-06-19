//! Keystroke handling for the friendly goal-setup modal.
//!
//! When `app.state.goal_prompt` is active, this consumes all key presses:
//! typing edits the current line, Enter adds a goal (or finishes on a blank
//! line), Backspace deletes, and Esc skips the prompt entirely.

use crossterm::event::{KeyCode, KeyEvent};

use crate::tui::app::state::App;

/// Handle a key press while the goal prompt is open.
///
/// Returns `true` if the key was consumed by the prompt.
pub(super) fn handle_goal_prompt_key(app: &mut App, key: KeyEvent) -> bool {
    if app.state.goal_prompt.is_none() {
        return false;
    }
    match key.code {
        KeyCode::Esc => finish(app),
        KeyCode::Enter => {
            let done = app
                .state
                .goal_prompt
                .as_mut()
                .map(|p| p.submit_line())
                .unwrap_or(true);
            if done {
                finish(app);
            }
        }
        KeyCode::Backspace => {
            if let Some(p) = app.state.goal_prompt.as_mut() {
                p.backspace();
            }
        }
        KeyCode::Char(c) => {
            if let Some(p) = app.state.goal_prompt.as_mut() {
                p.push_char(c);
            }
        }
        _ => {}
    }
    app.state.needs_redraw = true;
    true
}

fn finish(app: &mut App) {
    if let Some(mut p) = app.state.goal_prompt.take() {
        p.finish();
    }
    app.state.status = "Goals saved — getting to work.".to_string();
}
