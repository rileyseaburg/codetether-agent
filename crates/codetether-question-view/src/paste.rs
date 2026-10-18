//! Paste delivery to the active question runtime.
use codetether_question::{Action, runtime};
/// Insert pasted text into the current question's custom answer.
///
/// # Arguments
/// * `session_id` - Current session identifier.
/// * `text` - Text to append to the answer.
///
/// # Returns
/// Whether a pending question accepted the edit.
///
/// # Examples
/// ```
/// assert!(!codetether_question_view::paste(None, "answer"));
/// ```
pub fn paste(session_id: Option<&str>, text: &str) -> bool {
    session_id.is_some_and(|id| runtime::act(id, Action::Text(text.to_owned())))
}
