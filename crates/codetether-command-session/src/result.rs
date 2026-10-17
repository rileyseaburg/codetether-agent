//! Stable model-facing formatting for unified command output.

use super::{Poll, SpawnMetadata};
use codetether_tool_core::ToolResult;

#[path = "result/cache_guidance.rs"]
mod cache_guidance;
#[path = "result/redact.rs"]
mod redact;
#[path = "result/metadata.rs"]
mod result_metadata;
#[path = "result/status.rs"]
mod status;

/// Render one poll with execution metadata and optional persistent session ID.
///
/// # Arguments
/// * `poll` - Output, status and liveness from the latest poll.
/// * `metadata` - Host execution context and redactions.
/// * `id` - Persistent session identity, if registered.
///
/// # Returns
/// A shared tool result with redacted output and structured lifecycle metadata.
///
/// # Examples
/// ```rust
/// # fn render(poll: codetether_command_session::Poll, metadata: &codetether_command_session::SpawnMetadata) {
/// let result = codetether_command_session::tool_result(poll, metadata, Some(1));
/// assert_eq!(result.metadata["session_id"], 1);
/// # }
/// ```
pub fn tool_result(poll: Poll, metadata: &SpawnMetadata, id: Option<u64>) -> ToolResult {
    let heading = match id {
        Some(id) if poll.running => format!("Script running with session ID {id}"),
        Some(id) => format!(
            "Session {id} exited with code {}",
            poll.exit_code.unwrap_or(-1)
        ),
        None => format!("Process exited with code {}", poll.exit_code.unwrap_or(-1)),
    };
    let heading = format!("{heading}{}", status::suffix(&poll));
    let recent = redact::output(&poll.output, &metadata.redactions);
    let recent = cache_guidance::annotate(recent, metadata.sandboxed);
    let output = if recent.is_empty() {
        heading
    } else {
        format!("{heading}\n{recent}")
    };
    let result = ToolResult {
        output,
        success: poll.running || poll.exit_code == Some(0),
        metadata: result_metadata::build(&poll, metadata, id),
    };
    result
}

#[cfg(test)]
#[path = "result/tests.rs"]
mod tests;
