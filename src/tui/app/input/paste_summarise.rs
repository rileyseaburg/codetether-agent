//! Divert a large paste into the text sidecar buffer.

use crate::tui::app::state::App;
use crate::tui::models::InputMode;

/// Attach a large paste to the sidecar and insert a placeholder at
/// the cursor instead of the full text.
pub(super) fn attach_summarised_paste(app: &mut App, normalized: &str) {
    let line_count = normalized.lines().count();
    let size = super::super::pasted_text::format_size(normalized.len());
    let placeholder =
        super::super::pasted_text::attach_paste(&mut app.state, normalized.to_string());
    let id = app
        .state
        .pending_text_pastes
        .last()
        .map(|p| p.id)
        .unwrap_or(0);
    app.state.input_mode = if app.state.input.starts_with('/') {
        InputMode::Command
    } else {
        InputMode::Editing
    };
    app.state.insert_text(&placeholder);
    app.state.status = format!(
        "Pasted {line_count} lines ({size}) summarised as #{id}; full text will be sent to the agent."
    );
}
