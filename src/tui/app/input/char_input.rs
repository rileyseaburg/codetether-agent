//! Character input routing for the TUI.
//!
//! Dispatches printable characters to symbol search, bus
//! filter, model filter, file picker filter, or the chat
//! input buffer depending on the active view.
//!
//! # Examples
//!
//! ```ignore
//! handle_char(&mut app, modifiers, 'x').await;
//! ```

use crossterm::event::KeyModifiers;

use crate::tui::app::state::App;
use crate::tui::app::symbols::{refresh_symbol_search, symbol_search_active};
use crate::tui::models::{InputMode, ViewMode};

/// Route a printable character to the correct view handler.
///
/// Depending on the active view mode, the character is sent
/// to symbol search, bus filter, model filter, file picker
/// filter, or the chat input buffer.
///
/// # Examples
///
/// ```ignore
/// handle_char(&mut app, modifiers, 'x').await;
/// ```
pub async fn handle_char(app: &mut App, modifiers: KeyModifiers, c: char) {
    let no_mods =
        !modifiers.contains(KeyModifiers::CONTROL) && !modifiers.contains(KeyModifiers::ALT);

    if no_mods && symbol_search_active(app) {
        app.state.symbol_search.handle_char(c);
        refresh_symbol_search(app).await;
    } else if app.state.view_mode == ViewMode::Bus && app.state.bus_log.filter_input_mode && no_mods
    {
        app.state.bus_log.push_filter_char(c);
        app.state.status = format!("Protocol filter: {}", app.state.bus_log.filter);
    } else if app.state.view_mode == ViewMode::Model && no_mods {
        app.state.model_filter_push(c);
    } else if app.state.view_mode == ViewMode::FilePicker && no_mods {
        crate::tui::app::file_picker::file_picker_filter_push(app, c);
    } else if app.state.view_mode == ViewMode::Chat && no_mods {
        app.state.input_mode = if app.state.input.is_empty() && c == '/' {
            InputMode::Command
        } else if app.state.input.starts_with('/') || c == '/' {
            InputMode::Command
        } else {
            InputMode::Editing
        };
        app.state.insert_char(c);
    }
}
