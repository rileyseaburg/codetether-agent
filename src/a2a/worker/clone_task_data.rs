//! Clone-repo task input struct.

use std::path::Path;

use reqwest::Client;

use crate::a2a::worker_workspace_record::RegisteredWorkspaceRecord;

pub(super) struct CloneRepoTask<'a> {
    pub(super) client: &'a Client,
    pub(super) server: &'a str,
    pub(super) token: &'a Option<String>,
    pub(super) worker_id: &'a str,
    pub(super) workspace: &'a RegisteredWorkspaceRecord,
    pub(super) repo_path: &'a Path,
    pub(super) git_url: &'a str,
    pub(super) branch: &'a str,
    pub(super) temp_helper_path: &'a Path,
    pub(super) metadata: &'a serde_json::Map<String, serde_json::Value>,
}
