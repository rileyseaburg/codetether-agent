//! Codex-compatible V2 wait result rendering.

use crate::tool::ToolResult;
use crate::tool::agent::collaboration_runtime::parent_activity::Activity;
use serde_json::json;

pub(super) fn result(activity: Option<Activity>) -> ToolResult {
    let (message, timed_out) = match activity {
        Some(Activity::Mailbox) => ("Wait completed.", false),
        Some(Activity::Steered) => ("Wait interrupted by new input.", false),
        None => ("Wait timed out.", true),
    };
    ToolResult::success(json!({"message":message, "timed_out":timed_out}).to_string())
}

#[cfg(test)]
#[path = "outcome_tests.rs"]
mod tests;
