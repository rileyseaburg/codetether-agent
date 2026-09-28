//! Context-sensitive title for the chat input box.

use crate::tui::app::state::App;
use crate::tui::models::InputMode;

/// Builds the input title and environment-appropriate paste hint.
/// Inner widths below this (phone SSH clients) get a short title.
const COMPACT_WIDTH: u16 = 80;

pub(crate) fn build(app: &App, suffix: &str, width: u16) -> String {
    let compact = width < COMPACT_WIDTH;
    if app.state.processing {
        if compact {
            return format!(" Enter steers · Esc cancels{suffix}");
        }
        return format!(" Message (Processing — Enter steers turn · Esc cancels){suffix}");
    }
    if matches!(app.state.input_mode, InputMode::Command) {
        return format!(" Command (/ for commands, Tab to autocomplete){suffix}");
    }
    if compact {
        return format!(" Message (Enter send · / cmds · ? help){suffix}");
    }
    let paste = if crate::tui::clipboard::is_ssh_or_headless() {
        "Ctrl+Shift+V"
    } else {
        "Ctrl+V"
    };
    format!(
        " Message (Enter=send · {paste}=paste · Ctrl+O=copy reply · \
         Ctrl+⇧Y=copy all · Ctrl+R=voice){suffix}"
    )
}
