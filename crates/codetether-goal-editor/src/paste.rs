//! Paste multiline text only into an active goal draft.
use crate::Host;
use codetether_editor::edit::EditorEdit;
/// Paste normalized text only when a goal draft is active.
/// # Arguments
/// Receives editor access and pasted text.
/// # Returns
/// Whether the paste belongs to the goal editor.
/// # Examples
/// ```no_run
/// fn paste<H: codetether_goal_editor::Host>(host: &mut H) {
///     codetether_goal_editor::paste(host, "new objective");
/// }
/// ```
pub fn paste<H: Host>(app: &mut H, text: &str) -> bool {
    if app.draft().is_none() {
        return false;
    }
    if let Some(buffer) = app.buffer_mut() {
        let normalized = codetether_terminal_text::normalize_paste(text);
        for ch in normalized.chars() {
            buffer.backend_mut().insert_char(ch);
        }
        buffer.refresh_highlight();
    }
    app.redraw();
    true
}
