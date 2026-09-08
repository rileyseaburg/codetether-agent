//! Image loading is read-only; generation and arbitrary fixtures retain policy gates.

use crate::config::Config;
use crate::runtime_policy::{RuntimeToolPolicy, ToolKind, ToolPolicyOutcome};

#[test]
fn image_loading_is_allowed_without_authorizing_unknown_tools() {
    let _guard = crate::approval::test_env::lock_env();
    let policy = RuntimeToolPolicy::from_config(&Config::default());
    let decision = policy.decide_tool("image");
    assert_eq!(decision.tool_kind, ToolKind::ReadOnly);
    assert_eq!(decision.outcome, ToolPolicyOutcome::Allow);
    for tool in ["image_gen", "image_fixture"] {
        assert!(!super::is_read_only(tool));
        assert_eq!(
            policy.decide_tool(tool).outcome,
            ToolPolicyOutcome::RequireApproval
        );
    }
}
