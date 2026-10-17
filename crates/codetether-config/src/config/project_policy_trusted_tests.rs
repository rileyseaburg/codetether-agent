use super::sanitize_project_overlay;
use crate::config::{
    ApprovalPolicy, Config, PermissionAction, PermissionProfile, ProjectTrustLevel, SandboxMode,
};

#[test]
fn trusted_base_allows_project_policy_overlay() {
    let base: Config = toml::from_str(r#"trust_level = "trusted""#).unwrap();
    let merged = base
        .clone()
        .merge(sanitize_project_overlay(&base, project_overlay()));
    assert_eq!(
        merged.effective_sandbox_mode(),
        SandboxMode::DangerFullAccess
    );
    assert_eq!(merged.effective_approval_policy(), ApprovalPolicy::Never);
    assert_eq!(merged.effective_trust_level(), ProjectTrustLevel::Trusted);
    assert_eq!(
        merged.effective_permission_profile(),
        PermissionProfile::Disabled
    );
    assert_eq!(
        merged.permissions.tools.get("bash"),
        Some(&PermissionAction::Allow)
    );
}

fn project_overlay() -> Config {
    let mut cfg: Config = toml::from_str(
        r#"
sandbox_mode = "danger-full-access"
approval_policy = "never"
permission_profile = "disabled"
"#,
    )
    .unwrap();
    cfg.permissions
        .tools
        .insert("bash".into(), PermissionAction::Allow);
    cfg
}
