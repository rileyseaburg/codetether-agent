//! One isolated mux runtime hosted by a workspace server.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::{MuxRuntimeStatus, MuxWindow};

/// A named session: its own windows, active window, and TUI runtime state.
///
/// Sessions never share windows, runtime state, agent tasks, or durable
/// CodeTether sessions with one another, even when they share a checkout.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub(in crate::mux) struct MuxSession {
    pub name: String,
    pub active_window: u64,
    pub windows: Vec<MuxWindow>,
    #[serde(default)]
    pub runtime: Option<MuxRuntimeStatus>,
}

impl MuxSession {
    pub(in crate::mux) fn new(name: String, workspace: PathBuf) -> Self {
        Self {
            name,
            active_window: 0,
            windows: vec![MuxWindow::new(0, workspace)],
            runtime: None,
        }
    }

    /// The window that receives input and hosts the TUI.
    pub(in crate::mux) fn active(&self) -> Option<&MuxWindow> {
        self.windows
            .iter()
            .find(|window| window.id == self.active_window)
    }

    pub(in crate::mux) fn window(&self, id: u64) -> Option<&MuxWindow> {
        self.windows.iter().find(|window| window.id == id)
    }
}
