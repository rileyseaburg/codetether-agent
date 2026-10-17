//! Render a rejected task-stream connection response.
use super::summarize_response_body;

/// Build the connect error for a non-success stream response.
pub async fn connect_error(response: reqwest::Response) -> anyhow::Error {
    let status = response.status();
    let body = response
        .text()
        .await
        .unwrap_or_else(|error| format!("<failed to read response body: {error}>"));
    anyhow::anyhow!(
        "Failed to connect task stream: status={} body={}",
        status,
        summarize_response_body(&body)
    )
}
