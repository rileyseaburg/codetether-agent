//! Ctrl-key and special-key bindings for the TUI.
//!
//! Handles Ctrl-C/Q (quit), Ctrl-T (symbol search), Ctrl-B
//! (layout toggle), Ctrl-O (copy latest reply), Ctrl-R (voice input),
//! Ctrl-V (clipboard paste), Ctrl-X (watchdog cancel), Ctrl-G/G (scroll
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

use crate::tui::app::session_runtime::TuiSessionHandle;
use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

use super::alt_scroll::handle_alt_scroll;
use super::approval_key;
use super::clipboard::handle_clipboard_paste;
use super::copy_reply::handle_copy_reply;
use super::copy_transcript::handle_copy_transcript;
use super::ctrl_c::handle_ctrl_c;
use super::side_question::compose_side_question;

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
    _cwd: &Path,
    runtime: &TuiSessionHandle,
    key: KeyEvent,
) -> Option<anyhow::Result<bool>> {
    if let Some(r) = handle_alt_scroll(app, key) {
        return Some(r);
    }

    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    match key.code {
        KeyCode::Char('a') if ctrl && approval_key::approve(app) => {}
        KeyCode::Char('d') if ctrl && approval_key::deny(app) => {}
        KeyCode::Char('c') if ctrl => return Some(Ok(handle_ctrl_c(app, runtime))),
        KeyCode::Char('q') if ctrl => return Some(Ok(true)),
        KeyCode::Char('t') if ctrl => {
            app.state.symbol_search.open();
            app.state.status = "Symbol search".to_string();
        }
        KeyCode::Char('w') if ctrl && app.state.view_mode == ViewMode::Chat => {
            compose_side_question(app);
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
        KeyCode::Char('l') if ctrl && app.state.view_mode == ViewMode::Chat => {
            app.state.messages.clear();
            app.state.streaming_text.clear();
            app.state.status = "Screen cleared".to_string();
        }
        KeyCode::Char('s') if ctrl && app.state.view_mode == ViewMode::Chat => {
            app.state.save_scroll_state();
            app.state.view_mode = ViewMode::Swarm;
            app.state.status = "Swarm".to_string();
        }
        KeyCode::Char('p') if ctrl && app.state.view_mode == ViewMode::Chat => {
            app.state.save_scroll_state();
            app.state.view_mode = ViewMode::Protocol;
            app.state.status = "Protocol registry".to_string();
        }
        KeyCode::Char('o') if ctrl && app.state.view_mode == ViewMode::Chat => {
            handle_copy_reply(app);
        }
        KeyCode::Char('r') if ctrl && app.state.view_mode == ViewMode::Chat => {
            handle_voice_key(app);
        }
        KeyCode::Char('v') if ctrl && app.state.view_mode == ViewMode::Chat => {
            handle_clipboard_paste(app);
        }
        KeyCode::Insert
            if key.modifiers.contains(KeyModifiers::SHIFT)
                && app.state.view_mode == ViewMode::Chat =>
        {
            // Shift+Insert is the standard paste shortcut on Windows
            // and many Linux terminal emulators.
            handle_clipboard_paste(app);
        }
        KeyCode::Char('Y') if ctrl && app.state.view_mode == ViewMode::Chat => {
            // Kitty keyboard protocol: Ctrl+Shift+Y arrives as 'Y' + CONTROL.
            handle_copy_transcript(app);
        }
        KeyCode::Char('y') if ctrl && app.state.view_mode == ViewMode::Chat => {
            // Legacy terminals: Ctrl+Shift+Y arrives as 'y' + CONTROL+SHIFT.
            if key.modifiers.contains(KeyModifiers::SHIFT) {
                handle_copy_transcript(app);
            } else {
                handle_copy_reply(app);
            }
        }
        KeyCode::Char('x') if ctrl && app.state.watchdog_notification.is_some() => {
            crate::tui::app::watchdog::handle_watchdog_cancel(&mut app.state);
        }
        _ => return None,
    }
    Some(Ok(false))
}

#[cfg(not(test))]
fn handle_voice_key(app: &mut App) {
    super::voice::handle_voice_input(app);
}

#[cfg(test)]
fn handle_voice_key(app: &mut App) {
    app.state.status = "Voice shortcut".to_string();
}
