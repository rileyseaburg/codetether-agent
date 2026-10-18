//! Image loading is read-only; generation and arbitrary fixtures retain policy gates.

use crate::{RuntimeToolPolicy, ToolKind, ToolPolicyOutcome};
use codetether_config::Config;

#[test]
fn image_loading_is_allowed_without_authorizing_unknown_tools() {
    let _guard = codetether_native_approval::test_env::lock_env();
    let policy = RuntimeToolPolicy::from_config(&Config::default());
    let decision = policy.decide_tool("image");
    assert_eq!(decision.tool_kind, ToolKind::ReadOnly);
    assert_eq!(decision.outcome, ToolPolicyOutcome::Allow);
    for tool in ["image_gen", "image_fixture"] {
        assert!(!codetether_tool_core::readonly::is_read_only(tool));
        assert_eq!(
            policy.decide_tool(tool).outcome,
            ToolPolicyOutcome::RequireApproval
        );
    }
}

/// `skill` only reads `~/.codetether/skills`, `todoread` only reads the
/// session's task file, and `bus_inspect` only reads an in-memory ring buffer.
/// None of them should interrupt the user with a `mutating_tool` approval.
#[test]
fn inspection_tools_never_require_approval() {
    let _guard = codetether_native_approval::test_env::lock_env();
    let policy = RuntimeToolPolicy::from_config(&Config::default());
    for tool in ["skill", "todoread", "todo_read", "bus_inspect"] {
        let decision = policy.decide_tool(tool);
        assert_eq!(decision.tool_kind, ToolKind::ReadOnly, "{tool}");
        assert_eq!(decision.outcome, ToolPolicyOutcome::Allow, "{tool}");
    }
    assert!(!codetether_tool_core::readonly::is_read_only("todowrite"));
}
