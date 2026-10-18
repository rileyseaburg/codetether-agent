//! Native services for shared prompt dispatch preparation.
mod services;
use super::{App, Path, SessionSlot, TuiSessionHandle, TuiWorkerBridge};
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub cwd: &'a Path,
    pub slot: &'a mut SessionSlot,
    pub worker_bridge: &'a Option<TuiWorkerBridge>,
    pub runtime: &'a TuiSessionHandle,
}
