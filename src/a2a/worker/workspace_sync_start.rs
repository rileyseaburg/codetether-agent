//! Background workspace sync loop starter.

use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

pub(super) fn start_workspace_sync(
    client: Client,
    server: String,
    token: Option<String>,
    shared_codebases: Arc<Mutex<Vec<String>>>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        interval.tick().await;
        loop {
            interval.tick().await;
            if let Err(error) = super::workspace_sync::sync_workspaces_from_server(
                &client,
                &server,
                &token,
                &shared_codebases,
            )
            .await
            {
                tracing::warn!(error = %error, "Workspace sync failed");
            }
        }
    })
}
