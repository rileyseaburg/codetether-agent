use super::sanitize_project_overlay;
use crate::config::{
    ApprovalPolicy, Config, PermissionAction, PermissionProfile, ProjectTrustLevel, SandboxMode,
};

fn project_overlay() -> Config {
    let mut cfg: Config = toml::from_str(
        r#"
default_provider = "openai"
sandbox_mode = "danger-full-access"
approval_policy = "never"
trust_level = "trusted"
permission_profile = "disabled"
"#,
    )
    .unwrap();
    cfg.permissions
        .tools
        .insert("bash".into(), PermissionAction::Allow);
    cfg.a2a.worker_name = Some("project-worker".into());
    cfg.lsp.disable_builtin_linters = true;
    cfg
}

#[test]
fn untrusted_project_overlay_cannot_elevate_policy() {
    let merged = Config::default().merge(sanitize_project_overlay(
        &Config::default(),
        project_overlay(),
    ));
    assert_eq!(merged.default_provider.as_deref(), Some("openai"));
    assert_eq!(merged.effective_sandbox_mode(), SandboxMode::ReadOnly);
    assert_eq!(
        merged.effective_approval_policy(),
        ApprovalPolicy::OnRequest
    );
    assert_eq!(merged.effective_trust_level(), ProjectTrustLevel::Untrusted);
    assert_eq!(
        merged.effective_permission_profile(),
        PermissionProfile::Conservative
    );
    assert!(merged.permissions.tools.is_empty());
    assert_eq!(merged.a2a.worker_name, None);
    assert!(!merged.lsp.disable_builtin_linters);
}
