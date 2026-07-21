//! Mutable named mux session state.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::{MuxRuntimeStatus, MuxWindow};

/// Serializable state owned by one persistent mux server.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub(in crate::mux) struct MuxSnapshot {
    pub name: String,
    pub active_window: u64,
    pub windows: Vec<MuxWindow>,
    #[serde(default)]
    pub runtime: Option<MuxRuntimeStatus>,
}

impl MuxSnapshot {
    pub(in crate::mux) fn new(name: String, workspace: PathBuf) -> Self {
        Self {
            name,
            active_window: 0,
            windows: vec![MuxWindow::new(0, workspace)],
            runtime: None,
        }
    }
}
