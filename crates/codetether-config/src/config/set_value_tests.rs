use super::{AccessMode, ApprovalPolicy, Config, SandboxMode};

#[test]
fn set_value_accepts_access_mode_and_policy_keys() {
    let mut cfg = Config::default();

    cfg.set_value("access_mode", "approve").unwrap();
    assert_eq!(cfg.access_mode, Some(AccessMode::Approve));
    assert_eq!(cfg.effective_approval_policy(), ApprovalPolicy::OnFailure);

    cfg.set_value("sandbox_mode", "danger-full-access").unwrap();
    cfg.set_value("approval_policy", "never").unwrap();

    assert_eq!(cfg.effective_sandbox_mode(), SandboxMode::DangerFullAccess);
    assert_eq!(cfg.effective_approval_policy(), ApprovalPolicy::Never);
    assert_eq!(cfg.effective_access_mode(), Some(AccessMode::Full));
}
