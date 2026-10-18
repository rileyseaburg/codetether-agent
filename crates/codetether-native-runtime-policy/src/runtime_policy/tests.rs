use super::{RuntimeToolPolicy, ToolKind, ToolPolicyDecision, ToolPolicyOutcome};
use crate::config::{AccessMode, Config, PermissionProfile, PermissionProfileConfig};

fn configured(edit: impl FnOnce(&mut Config)) -> Config {
    let mut config = Config::default();
    edit(&mut config);
    config
}

fn decide(config: &Config, tool_name: &str) -> ToolPolicyDecision {
    RuntimeToolPolicy::from_config(config).decide_tool(tool_name)
}

#[test]
fn defaults_allow_read_only_tools() {
    let decision = decide(&Config::default(), "read");
    assert_eq!(decision.outcome, ToolPolicyOutcome::Allow);
    assert_eq!(decision.tool_kind, ToolKind::ReadOnly);
}

#[test]
fn full_access_mode_allows_mutating_tools() {
    let config = configured(|config| {
        config.access_mode = Some(AccessMode::Full);
    });
    assert_eq!(decide(&config, "bash").outcome, ToolPolicyOutcome::Allow);
}

#[test]
fn disabled_profile_denies_mutating_tools() {
    let config = configured(|config| {
        config.permission_profile =
            Some(PermissionProfileConfig::Named(PermissionProfile::Disabled));
    });
    assert_eq!(decide(&config, "bash").outcome, ToolPolicyOutcome::Deny);
}

#[test]
fn default_mutating_tool_requires_approval() {
    let decision = decide(&Config::default(), "apply_patch");
    assert_eq!(decision.outcome, ToolPolicyOutcome::RequireApproval);
    assert_eq!(decision.tool_kind, ToolKind::Mutating);
}
