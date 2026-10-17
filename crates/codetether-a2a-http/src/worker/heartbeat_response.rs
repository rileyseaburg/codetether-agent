//! Interpret an extended heartbeat response.
use anyhow::Result;
/// Accept successful responses and report server rejection errors.
///
/// # Errors
/// Returns an error for an unsuccessful HTTP response.
pub async fn check_extended_heartbeat(response: reqwest::Response, task_id: &str) -> Result<()> {
    if !response.status().is_success() {
        anyhow::bail!(
            "extended heartbeat rejected with {}: {}",
            response.status(),
            response.text().await.unwrap_or_default()
        );
    }
    let body = response
        .json::<serde_json::Value>()
        .await
        .unwrap_or_default();
    if body.get("success").and_then(|value| value.as_bool()) == Some(false) {
        tracing::debug!(target: "codetether_agent::a2a::worker::extended_heartbeat", task_id = %task_id, message = body.get("message").and_then(|value| value.as_str()).unwrap_or("no active run"), "Extended task heartbeat skipped");
    }
    Ok(())
}
