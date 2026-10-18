//! One independently rooted mux window.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Serializable window metadata shared with attached clients.
/// # Examples
/// ```
/// let window = codetether_mux_model::MuxWindow {
///     id: 0, title: "repo".into(), workspace: "/repo".into(),
/// };
/// assert_eq!(window.id, 0);
/// ```
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct MuxWindow {
    /// Server-wide window ID.
    pub id: u64,
    /// Display title derived from the workspace directory.
    pub title: String,
    /// Window working directory.
    pub workspace: PathBuf,
}

impl MuxWindow {
    pub(super) fn new(id: u64, workspace: PathBuf) -> Self {
        let title = workspace
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("workspace")
            .to_string();
        Self {
            id,
            title,
            workspace,
        }
    }
}
