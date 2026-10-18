//! Native state borrowed by background-update sequencing.
mod bindings;
pub(super) struct Native<'a> {
    pub app: &'a mut super::App,
    pub cwd: &'a super::Path,
    pub slot: &'a mut super::SessionSlot,
    pub bus_recorder: Option<&'a super::BusRecorder>,
    pub worker_bridge: &'a mut Option<super::TuiWorkerBridge>,
    pub event_rx: &'a mut super::mpsc::Receiver<super::SessionEvent>,
    pub notice_rx: Option<&'a mut super::mpsc::Receiver<super::SessionNotice>>,
}
