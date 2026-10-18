//! Main TUI event loop orchestration.
//!
//! Drives the terminal draw → event → dispatch cycle using
//! `tokio::select!` across terminal, session, result, watchdog
//! and tick channels.

mod bus_inbox;
mod driver;
mod io;
mod redraw;
mod select_args;
mod select_loop;
mod setup;
mod shutdown;
mod smart_retry;
mod smart_retry_submit;
mod terminal;
mod tick;
use codetether_tui_reactor as timers;
mod watchdog;
mod watchdog_retry;

pub(crate) use {io::LoopIo, select_args::SelectArgs, timers::Timers as LoopTimers};

use std::{path::Path, sync::Arc};

use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::sync::mpsc;

use crate::bus::recorder::BusRecorder;
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
    bus_recorder: Arc<BusRecorder>,
    worker_bridge: Option<TuiWorkerBridge>,
    event_rx: mpsc::Receiver<SessionEvent>,
    runtime: TuiSessionHandle,
    notice_rx: mpsc::Receiver<SessionNotice>,
) -> anyhow::Result<()> {
    let setup = setup::create();
    codetether_tui_reactor::driver::run(driver::Native {
        terminal,
        app,
        cwd,
        registry,
        slot,
        bus_recorder,
        worker_bridge,
        event_rx,
        runtime,
        notice_rx,
        setup,
        last_draw: None,
    })
    .await
}
