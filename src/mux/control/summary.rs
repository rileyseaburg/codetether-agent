//! Safe mux state exposed to in-process user interfaces.

use std::path::PathBuf;

use crate::mux::model::MuxSnapshot;
use crate::mux::registry::MuxRecord;

/// One mux window suitable for display outside the protocol layer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MuxWindowSummary {
    pub id: u64,
    pub title: String,
    pub workspace: PathBuf,
}

/// One named mux server and its latest window state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MuxSessionSummary {
    pub name: String,
    pub address: String,
    pub pid: u32,
    pub active_window: u64,
    pub windows: Vec<MuxWindowSummary>,
    pub reachable: bool,
}

impl MuxSessionSummary {
    pub(super) fn from_record(record: &MuxRecord, reachable: bool) -> Self {
        Self::new(record, &record.state, reachable)
    }

    pub(super) fn from_state(record: &MuxRecord, state: &MuxSnapshot) -> Self {
        Self::new(record, state, true)
    }

    fn new(record: &MuxRecord, state: &MuxSnapshot, reachable: bool) -> Self {
        let windows = state.windows.iter().map(MuxWindowSummary::from).collect();
        Self {
            name: record.name.clone(),
            address: record.address.to_string(),
            pid: record.pid,
            active_window: state.active_window,
            windows,
            reachable,
        }
    }
}

impl From<&crate::mux::model::MuxWindow> for MuxWindowSummary {
    fn from(window: &crate::mux::model::MuxWindow) -> Self {
        Self {
            id: window.id,
            title: window.title.clone(),
            workspace: window.workspace.clone(),
        }
    }
}
