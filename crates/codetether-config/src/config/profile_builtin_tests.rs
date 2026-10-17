use super::{AccessMode, ApprovalPolicy, Config, PermissionProfile, SandboxMode};

fn cfg(profile: &str) -> Config {
    toml::from_str(&format!(r#"permission_profile = "{profile}""#)).unwrap()
}

fn assert_profile(
    cfg: Config,
    profile: PermissionProfile,
    mode: AccessMode,
    sandbox: SandboxMode,
    approval: ApprovalPolicy,
) {
    assert_eq!(cfg.effective_permission_profile(), profile);
    assert_eq!(cfg.effective_access_mode(), Some(mode));
    assert_eq!(cfg.effective_sandbox_mode(), sandbox);
    assert_eq!(cfg.effective_approval_policy(), approval);
}

#[test]
fn codex_builtin_permission_profiles_imply_policy() {
    assert_profile(
        cfg(":read-only"),
        PermissionProfile::ReadOnly,
        AccessMode::Ask,
        SandboxMode::ReadOnly,
        ApprovalPolicy::OnRequest,
    );
    assert_profile(
        cfg(":workspace"),
        PermissionProfile::Workspace,
        AccessMode::Approve,
        SandboxMode::WorkspaceWrite,
        ApprovalPolicy::OnFailure,
    );
    assert_profile(
        cfg(":danger-full-access"),
        PermissionProfile::DangerFullAccess,
        AccessMode::Full,
        SandboxMode::DangerFullAccess,
        ApprovalPolicy::Never,
    );
}
