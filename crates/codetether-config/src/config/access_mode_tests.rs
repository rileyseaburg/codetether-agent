use super::{AccessMode, ApprovalPolicy, Config, PermissionProfile, SandboxMode};

fn cfg(input: &str) -> Config {
    toml::from_str(input).unwrap()
}

fn assert_policy(cfg: &Config, mode: AccessMode, sandbox: SandboxMode, approval: ApprovalPolicy) {
    assert_eq!(cfg.effective_access_mode(), Some(mode));
    assert_eq!(cfg.effective_sandbox_mode(), sandbox);
    assert_eq!(cfg.effective_approval_policy(), approval);
    assert_eq!(
        cfg.effective_permission_profile(),
        PermissionProfile::Conservative
    );
}

#[test]
fn access_mode_maps_named_modes_to_effective_policy() {
    assert_policy(
        &cfg(r#"access_mode = "ask""#),
        AccessMode::Ask,
        SandboxMode::ReadOnly,
        ApprovalPolicy::OnRequest,
    );
    assert_policy(
        &cfg(r#"access_mode = "approve""#),
        AccessMode::Approve,
        SandboxMode::WorkspaceWrite,
        ApprovalPolicy::OnFailure,
    );
    assert_policy(
        &cfg(r#"access_mode = "full""#),
        AccessMode::Full,
        SandboxMode::DangerFullAccess,
        ApprovalPolicy::Never,
    );
}

#[test]
fn explicit_low_level_policy_overrides_access_mode_piece() {
    let cfg = cfg(r#"
access_mode = "full"
approval_policy = "on-request"
"#);
    assert_eq!(cfg.effective_access_mode(), None);
    assert_eq!(cfg.effective_sandbox_mode(), SandboxMode::DangerFullAccess);
    assert_eq!(cfg.effective_approval_policy(), ApprovalPolicy::OnRequest);
}
