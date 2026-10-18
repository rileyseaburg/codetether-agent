//! Native bindings for shared key selection.
mod input;
mod state;
use super::{App, Arc, Path, ProviderRegistry, SessionSlot, TuiSessionHandle, TuiWorkerBridge};
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub cwd: &'a Path,
    pub slot: &'a mut SessionSlot,
    pub registry: &'a Option<Arc<ProviderRegistry>>,
    pub worker_bridge: &'a Option<TuiWorkerBridge>,
    pub runtime: &'a TuiSessionHandle,
}
