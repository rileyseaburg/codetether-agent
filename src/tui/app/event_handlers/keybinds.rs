//! Unmodified (non-Ctrl/Alt) keycode dispatch.
//!
//! Handles Esc, Tab, Enter, arrow keys, Backspace, and
//! character input after Ctrl/Alt keys have been filtered
//! in [`super::keyboard`].
//!
//! # Examples
//!
//! ```ignore
//! handle_unmodified_key(
//!     &mut app, cwd, &mut session, &reg,
//!     &bridge, &tx, &rtx, key,
//! ).await?;
//! ```

use std::path::Path;
use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use crate::tui::app::{input, navigation as nav, state::App, symbols};
use crate::tui::worker_bridge::TuiWorkerBridge;

use super::mode_keys::handle_char_or_mode_key;

/// Dispatch a key press that has no Ctrl/Alt modifier.
///
/// Called from [`super::handle_event`] after the modifier
/// keys have been checked.  Returns `Ok(false)` always
/// (quit is handled at the Ctrl-key level).
///
/// # Examples
///
/// ```ignore
/// handle_unmodified_key(
///     &mut app, cwd, &mut session, &reg,
///     &bridge, &tx, &rtx, key,
/// ).await?;
/// ```
pub(super) async fn handle_unmodified_key(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    event_tx: &mpsc::Sender<SessionEvent>,
    result_tx: &mpsc::Sender<anyhow::Result<Session>>,
    key: KeyEvent,
) -> anyhow::Result<bool> {
    match key.code {
        KeyCode::Esc => handle_esc(app),
        KeyCode::Tab if app.state.slash_suggestions_visible() => nav::handle_tab(app),
        KeyCode::Char('?') if app.state.view_mode != crate::tui::models::ViewMode::Chat => {
            nav::toggle_help(app)
        }
        KeyCode::Up => nav::handle_up(app, key.modifiers),
        KeyCode::Down => nav::handle_down(app, key.modifiers),
        KeyCode::PageUp => nav::handle_page_up(app),
        KeyCode::PageDown => nav::handle_page_down(app),
        KeyCode::Home => nav::handle_home(app),
        KeyCode::End => nav::handle_end(app),
        KeyCode::Left => nav::handle_left(app, key.modifiers),
        KeyCode::Right => nav::handle_right(app, key.modifiers),
        KeyCode::Delete => nav::handle_delete(app),
        KeyCode::Enter if symbols::symbol_search_active(app) => nav::handle_symbol_enter(app),
        KeyCode::Enter => {
            input::handle_enter(
                app,
                cwd,
                session,
                registry,
                worker_bridge,
                event_tx,
                result_tx,
            )
            .await;
        }
        _ => handle_char_or_mode_key(app, session, key).await,
    }
    Ok(false)
}

fn handle_esc(app: &mut App) {
    if app.state.watchdog_notification.is_some() {
        crate::tui::app::watchdog::handle_watchdog_dismiss(&mut app.state);
    } else {
        nav::handle_escape(app);
    }
}
