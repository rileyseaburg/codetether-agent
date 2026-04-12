//! Ctrl-key and special-key bindings for the TUI.
//!
//! Handles Ctrl-C/Q (quit), Ctrl-T (symbol search), Ctrl-B
//! (layout toggle), Ctrl-O (file picker), Ctrl-V (clipboard
//! image), Ctrl-X (watchdog cancel), Ctrl-G/G (scroll
//! top/bottom), and Alt-j/k/d/u (scroll by 1 or 5).
//!
//! # Examples
//!
//! ```ignore
//! if let Some(result) = handle_ctrl_key(&mut app, cwd, key) {
//!     return result;
//! }
//! ```

use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

use super::alt_scroll::handle_alt_scroll;
use super::clipboard::handle_clipboard_paste;
use super::copy_reply::handle_copy_reply;

/// Try to handle a Ctrl/Alt modified key press.
///
/// Returns `Some(Ok(true))` for quit, `Some(Ok(false))` if
/// the key was consumed, or `None` if it should fall through
/// to the regular keycode match.
///
/// # Examples
///
/// ```ignore
/// if let Some(result) = handle_ctrl_key(&mut app, cwd, key) {
///     return result;
/// }
/// ```
pub(super) fn handle_ctrl_key(
    app: &mut App,
    cwd: &Path,
    key: KeyEvent,
) -> Option<anyhow::Result<bool>> {
    if let Some(r) = handle_alt_scroll(app, key) {
        return Some(r);
    }

    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    match key.code {
        KeyCode::Char('c') if ctrl => return Some(Ok(true)),
        KeyCode::Char('q') if ctrl => return Some(Ok(true)),
        KeyCode::Char('t') if ctrl => {
            app.state.symbol_search.open();
            app.state.status = "Symbol search".to_string();
        }
        KeyCode::Char('s') if ctrl && app.state.view_mode == ViewMode::Chat => {
            if app.state.input.trim().is_empty() {
                app.state.input = "/steer ".to_string();
            } else if !app.state.input.starts_with("/steer") {
                app.state.input = format!("/steer {}", app.state.input.trim());
            }
            app.state.input_cursor = app.state.input.chars().count();
            app.state.status = "Compose queued steering for the next turn".to_string();
            app.state.refresh_slash_suggestions();
        }
        KeyCode::Char('b') if ctrl && app.state.view_mode == ViewMode::Chat => {
            app.state.chat_layout_mode = app.state.chat_layout_mode.toggle();
            let label = if app.state.chat_layout_mode
                == crate::tui::ui::webview::layout_mode::ChatLayoutMode::Webview
            {
                "Webview"
            } else {
                "Classic"
            };
            app.state.status = format!("Layout: {label}");
        }
        KeyCode::Char('o') if ctrl && app.state.view_mode == ViewMode::Chat => {
            crate::tui::app::file_picker::open_file_picker(app, cwd);
        }
        KeyCode::Char('v') if ctrl && app.state.view_mode == ViewMode::Chat => {
            handle_clipboard_paste(app);
        }
        KeyCode::Char('y') if ctrl && app.state.view_mode == ViewMode::Chat => {
            handle_copy_reply(app);
        }
        KeyCode::Char('x') if ctrl && app.state.watchdog_notification.is_some() => {
            crate::tui::app::watchdog::handle_watchdog_cancel(&mut app.state);
        }
        _ => return None,
    }
    Some(Ok(false))
}
