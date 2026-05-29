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
mod terminal;
mod tick;
mod timers;
mod watchdog;
mod watchdog_spawn;

pub(crate) use io::LoopIo;
pub(crate) use select_args::SelectArgs;
pub(crate) use timers::LoopTimers;

use std::{path::Path, sync::Arc};

use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::sync::mpsc;

use crate::bus::BusHandle;
use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
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
pub async fn run_event_loop(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    cwd: &Path,
    registry: Option<Arc<ProviderRegistry>>,
    session: &mut Session,
    bus_handle: &mut BusHandle,
    mut worker_bridge: Option<TuiWorkerBridge>,
    event_tx: mpsc::Sender<SessionEvent>,
    mut event_rx: mpsc::Receiver<SessionEvent>,
    result_tx: mpsc::Sender<anyhow::Result<Session>>,
    mut result_rx: mpsc::Receiver<anyhow::Result<Session>>,
) -> anyhow::Result<()> {
    let mut setup = setup::create();

    loop {
        tick::before_draw(app, &worker_bridge);
        crate::tui::app::safe_draw::draw_ui(terminal, app, session)?;
        let mut io = LoopIo::new(&mut event_rx, &mut result_rx, &mut setup.shutdown_rx);
        let mut args = SelectArgs {
            reader: &mut setup.reader,
            app,
            cwd,
            session,
            registry: &registry,
            worker_bridge: &mut worker_bridge,
            event_tx: &event_tx,
            result_tx: &result_tx,
            io: &mut io,
            timers: &mut setup.timers,
            bus_handle,
        };
        if select_loop::select_once(&mut args).await? {
            break;
        }
    }

    shutdown::deregister_bridge(&worker_bridge);
    Ok(())
}
