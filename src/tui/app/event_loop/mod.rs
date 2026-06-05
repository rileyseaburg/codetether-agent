//! Main TUI event loop orchestration.
//!
//! Drives the terminal draw → event → dispatch cycle using
//! `tokio::select!` across terminal, session, result, watchdog
//! and tick channels.  Each iteration redraws the UI, then
//! waits for whichever source fires first.
//!
//! # Examples
//!
//! ```ignore
//! run_event_loop(
//!     &mut terminal, &mut app, cwd, registry,
//!     &mut session, &mut bus, bridge, tx, rx, rtx, rrx,
//! ).await?;
//! ```

mod autochat;
mod bus_inbox;
mod coalesce;
mod io;
mod select_args;
mod select_loop;
mod setup;
mod shutdown;
mod smart_retry;
mod smart_retry_submit;
mod terminal;
mod tick;
mod timers;
mod watchdog;
mod watchdog_retry;

pub(crate) use {io::LoopIo, select_args::SelectArgs, timers::LoopTimers};

use std::{path::Path, sync::Arc};

use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::sync::mpsc;

use crate::bus::BusHandle;
use crate::provider::ProviderRegistry;
use crate::session::SessionEvent;
use crate::tui::app::session_runtime::{SessionNotice, SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

/// Drive the TUI draw-event-dispatch loop until quit.
///
/// Continuously redraws the UI, then uses `tokio::select!`
/// to multiplex terminal events, session events, results,
/// a watchdog stall timer, and a 50 ms background tick.
///
/// # Examples
///
/// ```ignore
/// run_event_loop(
///     &mut terminal, &mut app, cwd, registry,
///     &mut session, &mut bus, bridge, tx, rx, rtx, rrx,
/// ).await?;
/// ```
pub(crate) async fn run_event_loop(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    cwd: &Path,
    registry: Option<Arc<ProviderRegistry>>,
    slot: &mut SessionSlot,
    bus_handle: &mut BusHandle,
    mut worker_bridge: Option<TuiWorkerBridge>,
    mut event_rx: mpsc::Receiver<SessionEvent>,
    runtime: TuiSessionHandle,
    mut notice_rx: mpsc::Receiver<SessionNotice>,
) -> anyhow::Result<()> {
    let mut setup = setup::create();

    loop {
        tick::before_draw(app, &worker_bridge);
        if app.state.needs_redraw {
            crate::tui::app::safe_draw::draw_ui(terminal, app, slot.view())?;
            app.state.needs_redraw = false;
        }
        let mut io = LoopIo::new(&mut event_rx, &mut notice_rx, &mut setup.shutdown_rx);
        let mut args = SelectArgs {
            reader: &mut setup.reader,
            app,
            cwd,
            slot,
            registry: &registry,
            worker_bridge: &mut worker_bridge,
            runtime: &runtime,
            io: &mut io,
            timers: &mut setup.timers,
            bus_handle,
        };
        if select_loop::select_once(&mut args).await? {
            break;
        }
    }

    shutdown::deregister_bridge(&worker_bridge);
    runtime.shutdown().await;
    Ok(())
}
