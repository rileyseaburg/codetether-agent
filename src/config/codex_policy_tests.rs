use super::{ApprovalPolicy, Config, PermissionProfile, ProjectTrustLevel, SandboxMode};

#[test]
fn codex_policy_defaults_are_conservative() {
    let cfg = Config::default();
    assert_eq!(cfg.effective_sandbox_mode(), SandboxMode::ReadOnly);
    assert_eq!(cfg.effective_approval_policy(), ApprovalPolicy::OnRequest);
    assert_eq!(cfg.effective_trust_level(), ProjectTrustLevel::Untrusted);
    assert!(!cfg.is_project_trusted());
    assert_eq!(
        cfg.effective_permission_profile(),
        PermissionProfile::Conservative
    );
}

#[test]
fn parses_codex_policy_strings_and_profile_table() {
    let cfg: Config = toml::from_str(
        r#"
sandbox_mode = "workspace-write"
approval_policy = "never"
trust_level = "trusted"

[permission_profile]
type = "disabled"
"#,
    )
    .unwrap();
    assert_eq!(cfg.effective_sandbox_mode(), SandboxMode::WorkspaceWrite);
    assert_eq!(cfg.effective_approval_policy(), ApprovalPolicy::Never);
    assert_eq!(cfg.effective_trust_level(), ProjectTrustLevel::Trusted);
    assert!(cfg.is_project_trusted());
    assert_eq!(
        cfg.effective_permission_profile(),
        PermissionProfile::Disabled
    );
    let compact: Config = toml::from_str(r#"permission_profile = "disabled""#).unwrap();
    assert_eq!(
        compact.effective_permission_profile(),
        PermissionProfile::Disabled
    );
}

#[test]
fn merge_preserves_codex_policy_when_overlay_omits_it() {
    let base: Config = toml::from_str(r#"sandbox_mode = "danger-full-access""#).unwrap();
    let overlay: Config = toml::from_str(r#"default_provider = "openai""#).unwrap();
    let merged = base.merge(overlay);
    assert_eq!(
        merged.effective_sandbox_mode(),
        SandboxMode::DangerFullAccess
    );
    assert_eq!(merged.default_provider.as_deref(), Some("openai"));
}
