use super::blocked_with_config;
use crate::config::{Config, PermissionProfile, PermissionProfileConfig};

#[test]
fn read_only_tool_is_not_blocked() {
    assert!(blocked_with_config(&Config::default(), "read").is_none());
}

#[test]
fn mutating_tool_requires_approval_by_default() {
    let result = blocked_with_config(&Config::default(), "bash").unwrap();
    assert!(!result.success);
    assert_eq!(
        result.metadata.get("policy_outcome"),
        Some(&serde_json::json!("require_approval"))
    );
}

#[test]
fn disabled_profile_denies_mutating_tool() {
    let mut config = Config::default();
    config.permission_profile = Some(PermissionProfileConfig::Named(PermissionProfile::Disabled));
    let result = blocked_with_config(&config, "apply_patch").unwrap();
    assert_eq!(
        result.metadata.get("policy_outcome"),
        Some(&serde_json::json!("deny"))
    );
}
