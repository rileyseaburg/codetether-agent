//! Mode-specific character and backspace dispatch.
//!
//! Routes characters, Tab, and Backspace to the appropriate
//! view-mode handler after Ctrl/Alt and navigation keys have
//! been consumed by earlier layers.
//!
//! # Examples
//!
//! ```ignore
//! handle_char_or_mode_key(&mut app, &mut session, key).await;
//! ```

use crossterm::event::{KeyCode, KeyEvent};

use crate::session::Session;
use crate::tui::app::commands::toggle_auto_apply_edits;
use crate::tui::app::input::{
    handle_backspace, handle_bus_c, handle_bus_g, handle_bus_slash, handle_char,
    handle_sessions_char,
};
use crate::tui::app::settings::{toggle_network_access, toggle_slash_autocomplete};
use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

/// Dispatch a character or mode-specific key press.
///
/// Handles settings toggles, session filter, bus commands,
/// and general character input based on the active
/// [`ViewMode`].
///
/// # Examples
///
/// ```ignore
/// handle_char_or_mode_key(&mut app, &mut session, key).await;
/// ```
pub(super) async fn handle_char_or_mode_key(app: &mut App, session: &mut Session, key: KeyEvent) {
    match key.code {
        KeyCode::Char('a') if app.state.view_mode == ViewMode::Settings => {
            toggle_auto_apply_edits(app, session).await;
        }
        KeyCode::Char('n') if app.state.view_mode == ViewMode::Settings => {
            toggle_network_access(app, session).await;
        }
        KeyCode::Tab if app.state.view_mode == ViewMode::Settings => {
            toggle_slash_autocomplete(app, session).await;
        }
        KeyCode::Backspace if app.state.view_mode == ViewMode::Sessions => {
            app.state.session_filter_backspace();
        }
        KeyCode::Backspace => handle_backspace(app).await,
        KeyCode::Char('g') if app.state.view_mode == ViewMode::Bus => handle_bus_g(app),
        KeyCode::Char('c') if app.state.view_mode == ViewMode::Bus => handle_bus_c(app),
        KeyCode::Char('/') if app.state.view_mode == ViewMode::Bus => handle_bus_slash(app),
        KeyCode::Char(c) if app.state.view_mode == ViewMode::Sessions => {
            handle_sessions_char(app, key.modifiers, c);
        }
        KeyCode::Char(c) => handle_char(app, key.modifiers, c).await,
        _ => {}
    }
}
