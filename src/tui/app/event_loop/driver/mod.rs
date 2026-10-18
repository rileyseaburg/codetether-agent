//! Native resources retained by the shared interactive loop driver.
mod cycle;
mod finish;
mod prepare;
use crate::tui::{
    app::{
        session_runtime::{SessionNotice, SessionSlot, TuiSessionHandle},
        state::App,
    },
    worker_bridge::TuiWorkerBridge,
};
use crate::{bus::recorder::BusRecorder, provider::ProviderRegistry, session::SessionEvent};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{path::Path, sync::Arc, time::Instant};
use tokio::sync::mpsc;
pub(super) struct Native<'a> {
    pub terminal: &'a mut Terminal<CrosstermBackend<std::io::Stdout>>,
    pub app: &'a mut App,
    pub cwd: &'a Path,
    pub registry: Option<Arc<ProviderRegistry>>,
    pub slot: &'a mut SessionSlot,
    pub bus_recorder: Arc<BusRecorder>,
    pub worker_bridge: Option<TuiWorkerBridge>,
    pub event_rx: mpsc::Receiver<SessionEvent>,
    pub runtime: TuiSessionHandle,
    pub notice_rx: mpsc::Receiver<SessionNotice>,
    pub setup: super::setup::LoopSetup,
    pub last_draw: Option<Instant>,
}
