//! Status and redraw handling after goal-prompt input.
use crate::{GoalPromptState, KeyOutcome};
use crossterm::event::KeyEvent;

/// Apply a goal-prompt key and its UI feedback.
///
/// # Arguments
/// * `prompt` — Current optional goal prompt.
/// * `status` — Application status line, replaced when the prompt finishes.
/// * `needs_redraw` — Redraw flag set when an active prompt handles input.
/// * `key` — Terminal input event.
/// # Returns
/// Whether a prompt was active and consumed the key. Inactive prompts leave
/// the status and redraw flag unchanged.
/// # Examples
/// ```
/// let mut status = "ready".to_string();
/// let mut redraw = false;
/// let key = crossterm::event::KeyEvent::new(
///     crossterm::event::KeyCode::Enter, crossterm::event::KeyModifiers::NONE,
/// );
/// assert!(!codetether_goal_prompt::handle_key_with_feedback(&mut None, &mut status, &mut redraw, key));
/// assert_eq!(status, "ready");
/// assert!(!redraw);
/// ```
pub fn handle_key_with_feedback(
    prompt: &mut Option<GoalPromptState>,
    status: &mut String,
    needs_redraw: &mut bool,
    key: KeyEvent,
) -> bool {
    match crate::handle_key(prompt, key) {
        KeyOutcome::Inactive => return false,
        KeyOutcome::Updated => {}
        KeyOutcome::Finished => *status = "Goals saved — getting to work.".to_string(),
    }
    *needs_redraw = true;
    true
}
