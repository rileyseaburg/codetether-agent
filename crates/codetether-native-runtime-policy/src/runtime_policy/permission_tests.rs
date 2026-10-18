use super::{RuntimeToolPolicy, ToolPolicyOutcome};
use crate::config::{Config, PermissionAction};

#[test]
fn explicit_permission_tool_rule_overrides_profile() {
    let mut config = Config::default();
    config
        .permissions
        .tools
        .insert("bash".into(), PermissionAction::Deny);
    let decision = RuntimeToolPolicy::from_config(&config).decide_tool("bash");
    assert_eq!(decision.outcome, ToolPolicyOutcome::Deny);
}
