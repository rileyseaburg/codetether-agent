//! Host-visible results from a goal-prompt keystroke.
/// How the host should update status and redraw after an input event.
///
/// # Examples
/// ```
/// use codetether_goal_prompt::{handle_key,KeyOutcome};
/// use crossterm::event::{KeyCode,KeyEvent,KeyModifiers};
/// assert_eq!(handle_key(&mut None,KeyEvent::new(KeyCode::Enter,KeyModifiers::NONE)),KeyOutcome::Inactive);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyOutcome {
    /// No prompt was active; allow the application to handle the key.
    Inactive,
    /// An active prompt consumed the key and should be redrawn.
    Updated,
    /// The response was delivered and the prompt closed.
    Finished,
}
