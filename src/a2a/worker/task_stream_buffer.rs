//! Buffer framing, body summary, and cursor-path helpers for the task stream.

use crate::a2a::stream::cursor::Cursor;
use crate::a2a::stream::frame::parse_frame;

use super::{WorkerTaskRuntime, frame_handler::handle_frame};

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

/// Resume-cursor file path for a given worker id.
pub(super) fn cursor_path(worker_id: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("codetether-stream-cursor-{worker_id}"))
}

/// Collapse whitespace and truncate an error response body for logging.
pub(super) fn summarize_response_body(body: &str) -> String {
    const MAX_BODY_CHARS: usize = 512;
    let mut summary = body.split_whitespace().collect::<Vec<_>>().join(" ");
    if summary.chars().count() > MAX_BODY_CHARS {
        summary = summary.chars().take(MAX_BODY_CHARS).collect::<String>();
        summary.push_str("...");
    }
    summary
}

/// Drain complete `\n\n`-delimited SSE frames from `buffer` and handle each.
pub(super) async fn process_buffer(
    buffer: &mut Vec<u8>,
    runtime: &WorkerTaskRuntime,
    cursor: &mut Cursor,
) {
    while let Some(pos) = buffer.windows(2).position(|w| w == b"\n\n") {
        let event_bytes = buffer[..pos].to_vec();
        buffer.drain(..pos + 2);
        let Ok(event_str) = std::str::from_utf8(&event_bytes) else {
            continue;
        };
        if let Some(frame) = parse_frame(event_str) {
            handle_frame(&frame, runtime, cursor).await;
        }
    }
}
