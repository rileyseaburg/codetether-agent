use std::{path::Path, sync::Arc};

use crossterm::event::EventStream;
use tokio::sync::mpsc;

use crate::bus::BusHandle;
use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use crate::tui::{app::state::App, worker_bridge::TuiWorkerBridge};

pub(crate) struct SelectArgs<'a> {
    pub reader: &'a mut EventStream,
    pub app: &'a mut App,
    pub cwd: &'a Path,
    pub session: &'a mut Session,
    pub registry: &'a Option<Arc<ProviderRegistry>>,
    pub worker_bridge: &'a mut Option<TuiWorkerBridge>,
    pub event_tx: &'a mpsc::Sender<SessionEvent>,
    pub result_tx: &'a mpsc::Sender<anyhow::Result<Session>>,
    pub io: &'a mut super::LoopIo<'a>,
    pub timers: &'a mut super::LoopTimers,
    pub bus_handle: &'a mut BusHandle,
}
