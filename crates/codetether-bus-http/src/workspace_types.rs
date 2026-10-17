//! Control-plane workspace discovery records.
use super::*;

#[derive(Debug, Deserialize)]
pub(super) struct WorkspaceSummary {
    pub(super) id: String,
    pub(super) path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(super) struct WorkspaceDetails {
    pub(super) worker_id: Option<String>,
}
