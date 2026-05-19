//! Paste event handling for the TUI.
//!
//! Normalises line endings and dispatches pasted text to the
//! correct input buffer based on the active view mode.
//!
//! # Examples
//!
//! ```ignore
//! handle_paste(&mut app, "multi\nline").await;
//! ```

use crate::tui::app::state::App;
use crate::tui::app::symbols::{refresh_symbol_search, symbol_search_active};
use crate::tui::models::{InputMode, ViewMode};

/// Handle pasted text from the terminal.
///
/// Normalises line endings, then dispatches to symbol search,
/// bus filter, model filter, sessions filter, or chat input.
///
/// # Examples
///
/// ```ignore
/// handle_paste(&mut app, "pasted\ntext").await;
/// ```
pub async fn handle_paste(app: &mut App, text: &str) {
    let normalized = text.replace("\r\n", "\n").replace('\r', "\n");

    if symbol_search_active(app) {
        paste_chars_no_newlines(&normalized, |ch| app.state.symbol_search.handle_char(ch));
        refresh_symbol_search(app).await;
        return;
    }
    if app.state.view_mode == ViewMode::Bus && app.state.bus_log.filter_input_mode {
        paste_chars_no_newlines(&normalized, |ch| app.state.bus_log.push_filter_char(ch));
        app.state.status = format!("Protocol filter: {}", app.state.bus_log.filter);
        return;
    }
    if app.state.view_mode == ViewMode::Model {
        paste_chars_no_newlines(&normalized, |ch| app.state.model_filter_push(ch));
        return;
    }
    if app.state.view_mode == ViewMode::Sessions {
        paste_chars_no_newlines(&normalized, |ch| app.state.session_filter_push(ch));
        return;
    }
    if app.state.view_mode == ViewMode::Chat {
        paste_into_chat(app, &normalized);
    }
}

/// Iterate non-newline characters and feed them to `f`.
fn paste_chars_no_newlines(text: &str, mut f: impl FnMut(char)) {
    for ch in text.chars().filter(|ch| *ch != '\n') {
        f(ch);
    }
}

/// Insert pasted text into the chat input buffer.
fn paste_into_chat(app: &mut App, normalized: &str) {
    if super::try_attach_data_url(app, normalized) {
        return;
    }
    if super::pasted_text::should_summarize(normalized) {
        attach_summarised_paste(app, normalized);
        return;
    }
    app.state.input_mode = if app.state.input.is_empty() && normalized.starts_with('/') {
        InputMode::Command
    } else if app.state.input.starts_with('/') {
        InputMode::Command
    } else {
        InputMode::Editing
    };
    app.state.insert_text(normalized);
    let line_count = normalized.lines().count();
    app.state.status = if line_count > 1 {
        format!("Pasted {line_count} lines into input")
    } else {
        "Pasted into input".to_string()
    };
}

/// Attach a large paste to the sidecar and insert a placeholder at
/// the cursor instead of the full text.
fn attach_summarised_paste(app: &mut App, normalized: &str) {
    let line_count = normalized.lines().count();
    let byte_count = normalized.len();
    let size = super::pasted_text::format_size(byte_count);
    let placeholder = super::pasted_text::attach_paste(&mut app.state, normalized.to_string());
    let id = app
        .state
        .pending_text_pastes
        .last()
        .map(|p| p.id)
        .unwrap_or(0);
    if app.state.input.starts_with('/') {
        app.state.input_mode = InputMode::Command;
    } else {
        app.state.input_mode = InputMode::Editing;
    }
    app.state.insert_text(&placeholder);
    app.state.status = format!(
        "Pasted {line_count} lines ({size}) summarised as #{id}; full text will be sent to the agent."
    );
}
