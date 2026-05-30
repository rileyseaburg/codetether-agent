//! Task release helpers.

use anyhow::Result;
use reqwest::Client;

pub(super) async fn release_task_result(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    task_id: &str,
    status: &str,
    result: Option<String>,
    error: Option<String>,
    session_id: Option<String>,
    diagnostics: Option<serde_json::Value>,
) -> Result<()> {
    super::release::send(
        client,
        server,
        token,
        worker_id,
        super::release::Payload {
            task_id,
            status,
            result,
            error,
            session_id,
            diagnostics,
        },
    )
    .await
}
