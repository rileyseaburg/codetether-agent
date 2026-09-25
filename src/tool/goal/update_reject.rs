//! Model-visible result for a verifier-rejected goal transition.

use crate::tool::ToolResult;

/// Build the error returned when the verifier rejects a transition.
///
/// The goal stays active; `findings` tell the worker what remains.
pub(super) fn result(findings: &str) -> ToolResult {
    ToolResult::error(format!(
        "The independent verifier rejected this transition; the goal is still active. \
         Complete the remaining work below, then call update_goal again.\n\n{findings}"
    ))
}
