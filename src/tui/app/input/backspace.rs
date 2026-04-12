//! Backspace handling across TUI view modes.
//!
//! Deletes the character before the cursor in whichever
//! input buffer is currently active — symbol search, bus
//! filter, model filter, file picker, or chat.
//!
//! # Examples
//!
//! ```ignore
//! handle_backspace(&mut app).await;
//! ```

use crate::tui::app::state::App;
use crate::tui::app::symbols::{refresh_symbol_search, symbol_search_active};
use crate::tui::models::{InputMode, ViewMode};

/// Delete the character before the cursor.
///
/// Delegates to symbol search, bus filter, model filter,
/// file picker, or chat input depending on the active view.
/// Resets input mode to `Normal` when the chat input becomes
/// empty.
///
/// # Examples
///
/// ```ignore
/// handle_backspace(&mut app).await;
/// ```
pub async fn handle_backspace(app: &mut App) {
    if symbol_search_active(app) {
        app.state.symbol_search.handle_backspace();
        refresh_symbol_search(app).await;
    } else if app.state.view_mode == ViewMode::Bus && app.state.bus_log.filter_input_mode {
        app.state.bus_log.pop_filter_char();
        app.state.status = if app.state.bus_log.filter.is_empty() {
            "Protocol filter cleared".to_string()
        } else {
            format!("Protocol filter: {}", app.state.bus_log.filter)
        };
    } else if app.state.view_mode == ViewMode::Model {
        app.state.model_filter_backspace();
    } else if app.state.view_mode == ViewMode::FilePicker {
        crate::tui::app::file_picker::file_picker_filter_backspace(app);
    } else if app.state.view_mode == ViewMode::Chat {
        app.state.delete_backspace();
        if app.state.input.is_empty() {
            app.state.input_mode = InputMode::Normal;
        } else if app.state.input.starts_with('/') {
            app.state.input_mode = InputMode::Command;
        }
    }
}
