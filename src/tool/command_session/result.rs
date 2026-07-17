//! Stable model-facing formatting for unified command output.

use super::{Poll, SpawnMetadata};
use crate::tool::ToolResult;

#[path = "result/redact.rs"]
mod redact;
#[path = "result/metadata.rs"]
mod result_metadata;

pub(crate) fn tool_result(poll: Poll, metadata: &SpawnMetadata, id: Option<u64>) -> ToolResult {
    let heading = match id {
        Some(id) if poll.running => format!("Script running with session ID {id}"),
        Some(id) => format!(
            "Session {id} exited with code {}",
            poll.exit_code.unwrap_or(-1)
        ),
        None => format!("Process exited with code {}", poll.exit_code.unwrap_or(-1)),
    };
    let recent = redact::output(&poll.output, &metadata.redactions);
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
