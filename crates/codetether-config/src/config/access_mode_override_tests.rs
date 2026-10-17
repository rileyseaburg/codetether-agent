use super::{AccessMode, ApprovalPolicy, Config, SandboxMode};

#[test]
fn access_mode_override_replaces_low_level_policy() {
    let mut cfg: Config = toml::from_str(
        r#"
sandbox_mode = "read-only"
approval_policy = "on-request"
"#,
    )
    .unwrap();

    cfg.apply_access_mode_override(Some(AccessMode::Full));

    assert_eq!(cfg.effective_access_mode(), Some(AccessMode::Full));
    assert_eq!(cfg.effective_sandbox_mode(), SandboxMode::DangerFullAccess);
    assert_eq!(cfg.effective_approval_policy(), ApprovalPolicy::Never);
}
