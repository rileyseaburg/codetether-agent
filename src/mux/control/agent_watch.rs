//! Event-driven waiting for fresh output from one mux session.

use anyhow::Result;
use serde_json::json;

pub(crate) async fn watch_agent(name: &str, timeout_ms: u64) -> Result<Option<String>> {
    if super::agent_target::load(name).await?.is_none() {
        return Ok(None);
    }
    let mut output = super::subscribe_live_output();
    let wait = async {
        while let Some(frame) = output.recv().await {
            if frame.session == name {
                return Some(frame);
            }
        }
        None
    };
    let duration = std::time::Duration::from_millis(timeout_ms.clamp(250, 30_000));
    let frame = tokio::time::timeout(duration, wait).await.ok().flatten();
    let timed_out = frame.is_none();
    let status = super::agent_sessions()
        .await?
        .into_iter()
        .find(|item| item["name"] == name);
    Ok(Some(
        json!({"event": frame, "status": status, "timed_out": timed_out}).to_string(),
    ))
}
