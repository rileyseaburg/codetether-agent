//! One mux window suitable for display outside the protocol layer.

use std::path::PathBuf;

/// A window identifier, display title and working directory.
///
/// # Examples
/// ```
/// let window = codetether_mux_control::MuxWindowSummary {
///     id: 1, title: "shell".into(), workspace: ".".into(),
/// };
/// assert_eq!(window.id, 1);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MuxWindowSummary {
    /// Window identifier.
    pub id: u64,
    /// Window display title.
    pub title: String,
    /// Workspace directory.
    pub workspace: PathBuf,
}

impl From<&codetether_mux_model::MuxWindow> for MuxWindowSummary {
    fn from(window: &codetether_mux_model::MuxWindow) -> Self {
        Self {
            id: window.id,
            title: window.title.clone(),
            workspace: window.workspace.clone(),
        }
    }
}
