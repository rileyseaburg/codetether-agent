//! Goal-prompt keyboard editing and close handling.
use crate::{GoalPromptState, KeyOutcome};
use crossterm::event::{KeyCode, KeyEvent};
/// Apply a key and report whether the host should redraw or show completion.
///
/// # Arguments
/// * `prompt` - Optional active prompt, consumed when finished.
/// * `key` - Terminal input event.
///
/// # Returns
/// The host action required after editing or finishing the prompt.
///
/// # Examples
/// ```
/// use codetether_goal_prompt::{handle_key,KeyOutcome};
/// use crossterm::event::{KeyCode,KeyEvent,KeyModifiers};
/// assert_eq!(handle_key(&mut None,KeyEvent::new(KeyCode::Esc,KeyModifiers::NONE)),KeyOutcome::Inactive);
/// ```
pub fn handle_key(prompt: &mut Option<GoalPromptState>, key: KeyEvent) -> KeyOutcome {
    if prompt.is_none() {
        return KeyOutcome::Inactive;
    }
    let mut finished = false;
    match key.code {
        KeyCode::Esc => {
            finish(prompt);
            finished = true;
        }
        KeyCode::Enter => {
            let done = prompt.as_mut().map(|p| p.submit_line()).unwrap_or(true);
            if done {
                finish(prompt);
                finished = true;
            }
        }
        KeyCode::Backspace => {
            if let Some(p) = prompt.as_mut() {
                p.backspace();
            }
        }
        KeyCode::Char(c) => {
            if let Some(p) = prompt.as_mut() {
                p.push_char(c);
            }
        }
        _ => {}
    }
    if finished {
        KeyOutcome::Finished
    } else {
        KeyOutcome::Updated
    }
}
fn finish(prompt: &mut Option<GoalPromptState>) {
    if let Some(mut p) = prompt.take() {
        p.finish();
    }
}
