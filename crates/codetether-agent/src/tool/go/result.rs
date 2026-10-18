//! Convert the native Ralph result into the tool reporting fields.
use crate::cli::go_ralph::{GoRalphResult, format_go_ralph_result};
pub(super) fn summarize(result: GoRalphResult, task: &str) -> codetether_go_tool::Completion {
    let summary = format_go_ralph_result(&result, task);
    codetether_go_tool::Completion {
        passed: result.passed,
        total: result.total,
        all_passed: result.all_passed,
        feature_branch: result.feature_branch,
        summary,
    }
}
