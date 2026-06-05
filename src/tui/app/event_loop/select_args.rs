use std::{path::Path, sync::Arc};

use crossterm::event::EventStream;

use crate::bus::BusHandle;
use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::{app::state::App, worker_bridge::TuiWorkerBridge};

pub(crate) struct SelectArgs<'a> {
    pub reader: &'a mut EventStream,
    pub app: &'a mut App,
    pub cwd: &'a Path,
    pub slot: &'a mut SessionSlot,
    pub registry: &'a Option<Arc<ProviderRegistry>>,
    pub worker_bridge: &'a mut Option<TuiWorkerBridge>,
    pub runtime: &'a TuiSessionHandle,
    pub io: &'a mut super::LoopIo<'a>,
    pub timers: &'a mut super::LoopTimers,
    pub bus_handle: &'a mut BusHandle,
}
