//! One isolated mux runtime hosted by a workspace server.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::{MuxRuntimeStatus, MuxWindow};

/// A named session: its own windows, active window, and TUI runtime state.
///
/// Sessions never share windows, runtime state, agent tasks, or durable
/// CodeTether sessions with one another, even when they share a checkout.
/// # Examples
/// ```
/// let session = codetether_mux_model::MuxSession::new("work".into(), "/repo".into());
/// assert_eq!(session.active().unwrap().id, 0);
/// ```
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct MuxSession {
    /// Session name used for control requests.
    pub name: String,
    /// ID of the window receiving input.
    pub active_window: u64,
    /// Windows owned exclusively by this session.
    pub windows: Vec<MuxWindow>,
    /// Optional semantic state reported by the owning TUI.
    #[serde(default)]
    pub runtime: Option<MuxRuntimeStatus>,
}

impl MuxSession {
    /// Create a session with window zero rooted at the requested workspace.
    pub fn new(name: String, workspace: PathBuf) -> Self {
        Self {
            name,
            active_window: 0,
            windows: vec![MuxWindow::new(0, workspace)],
            runtime: None,
        }
    }

    /// The window that receives input and hosts the TUI.
    pub fn active(&self) -> Option<&MuxWindow> {
        self.windows
            .iter()
            .find(|window| window.id == self.active_window)
    }

    /// Find a window by ID within this session.
    pub fn window(&self, id: u64) -> Option<&MuxWindow> {
        self.windows.iter().find(|window| window.id == id)
    }
}
