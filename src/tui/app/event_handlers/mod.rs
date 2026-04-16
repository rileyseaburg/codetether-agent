//! Keyboard, mouse and paste event dispatch for the TUI.
//!
//! Routes raw crossterm events to the appropriate handler
//! based on the current [`ViewMode`], modifier keys, and
//! active UI overlays (help, symbol search, slash suggestions).
//!
//! # Examples
//!
//! ```ignore
//! let quit = handle_event(
//!     &mut app, cwd, &mut session, &registry,
//!     &bridge, &tx, &rtx, key,
//! ).await?;
//! ```

mod alt_scroll;
mod clipboard;
mod copy_reply;
mod keybinds;
mod keyboard;
mod mode_keys;
mod mouse;
mod okr;
mod okr_save;
mod overlay_scroll;
mod paste;
mod scroll_down;
mod scroll_up;
mod tests;

use std::path::Path;
use std::sync::Arc;

use crossterm::event::{KeyEvent, KeyEventKind};
use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

use keybinds::handle_unmodified_key;
use keyboard::handle_ctrl_key;

pub use mouse::handle_mouse_event;
pub use paste::handle_paste_event;

/// Dispatch a single key press to the appropriate handler.
///
/// Ctrl/Alt modified keys are routed through
/// [`keyboard::handle_ctrl_key`] first.  Unmodified keys
/// fall through to the `match key.code` below.  Returns
/// `Ok(true)` when the user requests quit.
///
/// # Examples
///
/// ```ignore
/// if handle_event(&mut app, cwd, &mut session, &reg,
///     &bridge, &tx, &rtx, key).await? {
///     break;
/// }
/// ```
pub async fn handle_event(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    event_tx: &mpsc::Sender<SessionEvent>,
    result_tx: &mpsc::Sender<anyhow::Result<Session>>,
    key: KeyEvent,
) -> anyhow::Result<bool> {
    if key.kind != KeyEventKind::Press {
        return Ok(false);
    }

    if let Some(result) = handle_ctrl_key(app, cwd, key) {
        return result;
    }

    handle_unmodified_key(
        app,
        cwd,
        session,
        registry,
        worker_bridge,
        event_tx,
        result_tx,
        key,
    )
    .await
}
