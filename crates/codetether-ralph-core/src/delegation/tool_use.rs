//! tool use policy for a Ralph relay role.
use super::*;

pub(super) fn tool_use_for_role(role: &str) -> ToolUse {
    if role.contains("coder") || role.contains("tester") {
        ToolUse::Yes
    } else {
        ToolUse::No
    }
}
