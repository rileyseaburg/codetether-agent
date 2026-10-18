//! Projection of one session into a frontend summary.
use super::{MuxSessionSummary, MuxWindowSummary};
use codetether_mux_model::{MuxSession, MuxSnapshot};
use codetether_mux_registry::MuxRecord;

impl MuxSessionSummary {
    pub(super) fn new(
        record: &MuxRecord,
        state: &MuxSnapshot,
        session: &MuxSession,
        reachable: bool,
    ) -> Self {
        Self {
            name: session.name.clone(),
            workspace: state.workspace.clone(),
            address: record.address.to_string(),
            pid: record.pid,
            active_window: session.active_window,
            windows: session.windows.iter().map(MuxWindowSummary::from).collect(),
            reachable,
            runtime: session.runtime.clone(),
        }
    }
}
