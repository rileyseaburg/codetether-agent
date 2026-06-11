//! Success response construction for patch execution.

use super::apply::ApplyOutcome;
use crate::tool::ToolResult;
use serde_json::json;

pub(super) fn build(outcome: &ApplyOutcome, dry_run: bool) -> ToolResult {
    let action = if dry_run { "Would modify" } else { "Modified" };
    let summary = format!(
        "{} {} files:\n{}",
        action,
        outcome.files_written.len(),
        outcome.messages.join("\n")
    );
    ToolResult::success(summary).with_metadata("files", json!(outcome.files_written))
}
