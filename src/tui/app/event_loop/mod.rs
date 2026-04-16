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
mod select_loop;
mod shutdown;
mod smart_retry;
mod terminal;
mod watchdog;
mod watchdog_spawn;

use std::{path::Path, sync::Arc, time::Duration};

use crossterm::event::EventStream;
use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::sync::mpsc;

use crate::bus::BusHandle;
use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use crate::tui::app::{state::App, worker_bridge::sync_worker_bridge_agents};
use crate::tui::{
    constants::MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS, ui::main::ui, worker_bridge::TuiWorkerBridge,
};

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
    let mut reader = EventStream::new();
    let tick = Duration::from_millis(50);
    let mut tick_timer = tokio::time::interval(tick);
    let wd_interval = Duration::from_secs(MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS);
    let mut wd_timer = tokio::time::interval(wd_interval);

    loop {
        sync_worker_bridge_agents(app, &worker_bridge);
        terminal.draw(|f| ui(f, app, session))?;
        if select_loop::select_once(
            &mut reader,
            app,
            cwd,
            session,
            &registry,
            &mut worker_bridge,
            &event_tx,
            &result_tx,
            &mut event_rx,
            &mut result_rx,
            &mut wd_timer,
            wd_interval,
            &mut tick_timer,
            bus_handle,
        )
        .await?
        {
            break;
        }
    }

    shutdown::deregister_bridge(&worker_bridge);
    Ok(())
}
