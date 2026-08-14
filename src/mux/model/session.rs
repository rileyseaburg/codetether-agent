//! Mutable named mux session state.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::{MuxRuntimeStatus, MuxWindow};
use crate::mux::isolation::Isolation;

/// Serializable state owned by one persistent mux server.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub(in crate::mux) struct MuxSnapshot {
    pub name: String,
    pub active_window: u64,
    pub windows: Vec<MuxWindow>,
    #[serde(default)]
    pub runtime: Option<MuxRuntimeStatus>,
    /// Workspace allocation policy applied to new windows.
    #[serde(default)]
    pub isolation: Isolation,
}

impl MuxSnapshot {
    pub(in crate::mux) fn new(name: String, workspace: PathBuf, isolation: Isolation) -> Self {
        Self {
            name,
            active_window: 0,
            windows: vec![MuxWindow::new(0, workspace)],
            runtime: None,
            isolation,
        }
    }
}
