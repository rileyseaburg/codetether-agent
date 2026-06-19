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

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::app::{input, navigation as nav, state::App, symbols};
use crate::tui::worker_bridge::TuiWorkerBridge;

use super::{mode_keys::handle_char_or_mode_key, paste_burst::enter_is_likely_paste_newline};

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
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
    key: KeyEvent,
) -> anyhow::Result<bool> {
    match key.code {
        KeyCode::Esc => handle_esc(app),
        KeyCode::Tab if app.state.view_mode == crate::tui::models::ViewMode::Chat => {
            nav::handle_tab(app)
        }
        KeyCode::Char('?') if app.state.view_mode != crate::tui::models::ViewMode::Chat => {
            nav::toggle_help(app)
        }
        KeyCode::Char('m') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            super::model_picker_key::open_model_picker_if_session(app, slot, registry).await
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
        // Shift+Enter / Alt+Enter inserts a literal newline into the
        // chat input instead of submitting. Must come BEFORE the bare
        // `KeyCode::Enter` arm. Shift+Enter requires the terminal to
        // report modifier bits on Enter (kitty keyboard protocol or
        // xterm modifyOtherKeys — see `PushKeyboardEnhancementFlags`
        // in run.rs); Alt+Enter is universally distinguishable.
        KeyCode::Enter
            if key.modifiers.contains(KeyModifiers::SHIFT)
                || key.modifiers.contains(KeyModifiers::ALT) =>
        {
            app.state.insert_char('\n');
        }
        KeyCode::Enter => {
            if enter_is_likely_paste_newline(app) {
                app.state.insert_char('\n');
            } else {
                input::handle_enter(app, cwd, slot, registry, worker_bridge, runtime).await;
            }
        }
        _ => handle_char_or_mode_key(app, slot, key).await,
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
