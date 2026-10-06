//! Multiline paste goes to the goal draft, never chat or an attached file.

use crate::tui::app::state::App;
use crate::tui::ui::editor::edit::EditorEdit;

pub(crate) fn paste(app: &mut App, text: &str) -> bool {
    if app.state.goal_editor.is_none() {
        return false;
    }
    if let Some(buffer) = app.state.editor.as_mut() {
        let normalized = crate::tui::chat::strip::normalize_paste(text);
        for ch in normalized.chars() {
            buffer.backend_mut().insert_char(ch);
        }
        buffer.refresh_highlight();
    }
    app.state.needs_redraw = true;
    true
}
