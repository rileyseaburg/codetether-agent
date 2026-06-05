//! Final TUI event-loop handoff.

use ratatui::{Terminal, backend::CrosstermBackend};

use crate::bus::BusHandle;
use crate::provider::ProviderRegistry;
use crate::session::Session;
use crate::tui::app::event_loop::run_event_loop;
use crate::tui::app::session_runtime::{self, SessionSlot};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

use super::channels::SessionChannels;

pub(super) async fn run(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    cwd: &std::path::Path,
    registry: Option<std::sync::Arc<ProviderRegistry>>,
    session: Session,
    bus_handle: &mut BusHandle,
    worker_bridge: Option<TuiWorkerBridge>,
    channels: SessionChannels,
) -> anyhow::Result<()> {
    let runtime = session_runtime::spawn(channels.event_tx, channels.notice_tx);
    let mut slot = SessionSlot::new(session);
    run_event_loop(
        terminal,
        app,
        cwd,
        registry,
        &mut slot,
        bus_handle,
        worker_bridge,
        channels.event_rx,
        runtime,
        channels.notice_rx,
    )
    .await
}
