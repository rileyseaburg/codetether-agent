//! Inner select!-based loop body.
//!
//! Contains the `tokio::select!` that multiplexes terminal,
//! session, result, watchdog and tick channels.
//!
//! # Examples
//!
//! ```ignore
//! let quit = select_once(&mut ctx).await?;
//! ```

use std::sync::Arc;
use std::time::Duration;

use crossterm::event::EventStream;
use futures::StreamExt;
use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

/// Run one iteration of the select! loop.
///
/// Returns `true` when the user requests quit.
///
/// # Examples
///
/// ```ignore
/// if select_once(&mut reader, app, cwd, session, &reg,
///     &bridge, &tx, &rtx, &mut erx, &mut rrx,
///     &mut watchdog, interval, tick).await? { break; }
/// ```
pub(super) async fn select_once(
    reader: &mut EventStream,
    app: &mut App,
    cwd: &std::path::Path,
    session: &mut Session,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &mut Option<TuiWorkerBridge>,
    event_tx: &mpsc::Sender<SessionEvent>,
    result_tx: &mpsc::Sender<anyhow::Result<Session>>,
    event_rx: &mut mpsc::Receiver<SessionEvent>,
    result_rx: &mut mpsc::Receiver<anyhow::Result<Session>>,
    watchdog_timer: &mut tokio::time::Interval,
    watchdog_interval: Duration,
    tick_timer: &mut tokio::time::Interval,
    bus_handle: &mut crate::bus::BusHandle,
) -> anyhow::Result<bool> {
    tokio::select! {
        maybe = reader.next() => {
            if super::terminal::handle_terminal_event(app, cwd, session, registry, worker_bridge, event_tx, result_tx, maybe).await? {
                return Ok(true);
            }
        }
        Some(evt) = event_rx.recv() => {
            crate::tui::app::session_events::handle_session_event(app, session, worker_bridge, evt).await;
        }
        Some(result) = result_rx.recv() => {
            crate::tui::app::background::apply_single_result(app, cwd, session, worker_bridge, result).await;
            super::smart_retry::execute_smart_switch_retry(app, session, registry, event_tx, result_tx).await;
        }
        _ = watchdog_timer.tick() => {
            super::watchdog::maybe_watchdog_restart(app, session, registry, event_tx, result_tx, watchdog_interval).await;
        }
        _ = tick_timer.tick() => {
            super::autochat::drain_autochat(app);
        }
    }
    crate::tui::app::background::drain_background_updates(
        app,
        cwd,
        session,
        bus_handle,
        worker_bridge,
        event_rx,
        result_rx,
    )
    .await;
    Ok(false)
}
