//! Safe mux state exposed to in-process user interfaces.

use std::path::PathBuf;

pub(crate) use super::summary_window::MuxWindowSummary;
use crate::mux::MuxRuntimeStatus;

/// One named mux session and the server hosting it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MuxSessionSummary {
    pub name: String,
    pub workspace: PathBuf,
    pub address: String,
    pub pid: u32,
    pub active_window: u64,
    pub windows: Vec<MuxWindowSummary>,
    pub reachable: bool,
    pub runtime: Option<MuxRuntimeStatus>,
}
