//! One independently rooted mux window.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Serializable window metadata shared with attached clients.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub(in crate::mux) struct MuxWindow {
    pub id: u64,
    pub title: String,
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
