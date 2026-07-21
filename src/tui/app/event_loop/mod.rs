//! Main TUI event loop orchestration.
//!
//! Drives the terminal draw → event → dispatch cycle using
//! `tokio::select!` across terminal, session, result, watchdog
//! and tick channels.

mod autochat;
mod bus_inbox;
mod coalesce;
mod io;
mod redraw;
mod resilient;
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
    let mut last_draw = None;
    loop {
        tick::before_draw(app, &worker_bridge, &mut setup.worker_sync_cursor);
        setup.mux_status.update(app, slot.view()).await;
        redraw::draw_if_ready(terminal, app, slot.view(), &mut last_draw)?;
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
        if resilient::run_resilient(&mut args).await {
            break;
        }
    }

    shutdown::finish(&worker_bridge, &runtime, setup.mux_status).await;
    Ok(())
}
