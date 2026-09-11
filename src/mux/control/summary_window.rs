//! One mux window suitable for display outside the protocol layer.

use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MuxWindowSummary {
    pub id: u64,
    pub title: String,
    pub workspace: PathBuf,
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
