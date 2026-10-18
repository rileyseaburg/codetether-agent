//! Question key dispatch, preserving global cancel and quit bindings.
use codetether_question::{Action, runtime};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
/// Dispatch a key to the active question and notify the host of UI changes.
///
/// # Arguments
/// * `session_id` - Current session identifier.
/// * `key` - Terminal event.
/// * `on_redraw` - Called before processing a non-control key.
/// * `on_activity` - Called after submitting a draft edit to the runtime.
///
/// # Returns
/// Whether the modal consumed the key. Global cancel and quit remain unhandled.
///
/// # Examples
/// ```
/// use crossterm::event::{KeyCode,KeyEvent,KeyModifiers};
/// let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
/// assert!(!codetether_question_view::handle_key(None, key, || {}, || {}));
/// ```
pub fn handle_key(
    session_id: Option<&str>,
    key: KeyEvent,
    on_redraw: impl FnOnce(),
    on_activity: impl FnOnce(),
) -> bool {
    let Some(id) = session_id else {
        return false;
    };
    let Some(draft) = runtime::pending(id) else {
        return false;
    };
    // Preserve the global turn-cancel and quit keys.
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        return !matches!(key.code, KeyCode::Char('c' | 'q'));
    }
    on_redraw();
    let action = match key.code {
        KeyCode::Esc => {
            runtime::dismiss(id);
            return true;
        }
        KeyCode::Up => Action::Previous,
        KeyCode::Down | KeyCode::Tab => Action::Next,
        KeyCode::BackTab => Action::Back,
        KeyCode::Enter => Action::Submit,
        KeyCode::Backspace => Action::Backspace,
        KeyCode::Char(' ')
            if draft.cursor() < draft.question().options.len() && draft.question().multiple =>
        {
            Action::Toggle
        }
        KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::ALT) => {
            Action::Text(c.to_string())
        }
        _ => return true,
    };
    runtime::act(id, action);
    on_activity();
    true
}
