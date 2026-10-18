//! Native bindings for shared event delivery order.
use crate::{
    session::SessionEvent,
    tui::{
        app::{session_runtime::SessionSlot, state::App},
        worker_bridge::TuiWorkerBridge,
    },
};
mod stages;
mod view;
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub slot: &'a mut SessionSlot,
    pub worker_bridge: &'a Option<TuiWorkerBridge>,
}
