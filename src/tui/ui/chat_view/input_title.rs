//! Context-sensitive title for the chat input box.

use crate::tui::app::state::App;
use crate::tui::models::InputMode;

/// Builds the input title and environment-appropriate paste hint.
pub(crate) fn build(app: &App, suffix: &str) -> String {
    if app.state.processing {
        return format!(" Message (Processing — Enter steers turn · Esc cancels){suffix}");
    }
    if matches!(app.state.input_mode, InputMode::Command) {
        return format!(" Command (/ for commands, Tab to autocomplete){suffix}");
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
