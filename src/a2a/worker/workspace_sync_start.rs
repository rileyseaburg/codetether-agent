//! Supply repository setup to shared workspace synchronization scheduling.
use reqwest::Client;
use std::sync::Arc;
use tokio::{sync::Mutex, task::JoinHandle};
pub(super) fn start_workspace_sync(
    client: Client,
    server: String,
    token: Option<String>,
    shared_codebases: Arc<Mutex<Vec<String>>>,
) -> JoinHandle<()> {
    codetether_a2a_http::workspace::start_workspace_sync(
        client,
        server,
        token,
        shared_codebases,
        super::workspace_sync_git::maybe_configure_repo_git_auth_from_entry,
    )
}
