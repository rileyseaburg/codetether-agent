//! Native successful-completion bindings and session borrow ownership.
mod active;
mod runtime;
use super::{App, Path, Session, SessionSlot, TuiWorkerBridge};

pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub cwd: &'a Path,
    pub slot: &'a mut SessionSlot,
    pub worker_bridge: &'a mut Option<TuiWorkerBridge>,
}
