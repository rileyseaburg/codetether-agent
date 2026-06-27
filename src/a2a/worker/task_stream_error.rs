//! Connect-error formatting for the worker task stream.

/// Build the connect error for a non-success stream response.
pub(super) async fn connect_error(response: reqwest::Response) -> anyhow::Error {
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

/// Collapse whitespace and truncate an error response body for logging.
fn summarize_response_body(body: &str) -> String {
    const MAX_BODY_CHARS: usize = 512;
    let mut summary = body.split_whitespace().collect::<Vec<_>>().join(" ");
    if summary.chars().count() > MAX_BODY_CHARS {
        summary = summary.chars().take(MAX_BODY_CHARS).collect::<String>();
        summary.push_str("...");
    }
    summary
}
