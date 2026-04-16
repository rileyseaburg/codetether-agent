//! Terminal event dispatch (key, paste, mouse, resize).
//!
//! Unwraps the crossterm `Event` and routes it to the
//! appropriate handler.  Returns `Ok(true)` when the user
//! requests quit.
//!
//! # Examples
//!
//! ```ignore
//! if handle_terminal_event(&mut app, cwd, &mut session,
//!     &reg, &bridge, &tx, &rtx, maybe).await? { break; }
//! ```

use std::path::Path;
use std::sync::Arc;

use crossterm::event::Event;
use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use crate::tui::app::event_handlers::{handle_event, handle_mouse_event, handle_paste_event};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

/// Dispatch a single terminal event.
///
/// Returns `Ok(true)` when the user presses quit, `Ok(false)`
/// otherwise, or propagates errors.
///
/// # Examples
///
/// ```ignore
/// let quit = handle_terminal_event(
///     &mut app, cwd, &mut session, &reg,
///     &bridge, &tx, &rtx, maybe,
/// ).await?;
/// ```
pub(super) async fn handle_terminal_event(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    event_tx: &mpsc::Sender<SessionEvent>,
    result_tx: &mpsc::Sender<anyhow::Result<Session>>,
    maybe: Option<Result<Event, std::io::Error>>,
) -> anyhow::Result<bool> {
    match maybe {
        Some(Ok(Event::Key(key))) => {
            handle_event(
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
        Some(Ok(Event::Paste(text))) => {
            handle_paste_event(app, &text).await;
            Ok(false)
        }
        Some(Ok(Event::Mouse(mouse))) => {
            handle_mouse_event(app, mouse);
            Ok(false)
        }
        _ => Ok(false),
    }
}
