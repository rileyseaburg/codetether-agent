use super::{AccessMode, ApprovalPolicy, Config, PermissionProfile, SandboxMode};

fn cfg(input: &str) -> Config {
    toml::from_str(input).unwrap()
}

#[test]
fn allowed_access_modes_fallback_to_first_allowed_mode() {
    let cfg = cfg(r#"
access_mode = "full"

[requirements]
allowed_access_modes = ["approve"]
"#);
    assert_eq!(cfg.effective_access_mode(), Some(AccessMode::Approve));
    assert_eq!(cfg.effective_sandbox_mode(), SandboxMode::WorkspaceWrite);
    assert_eq!(cfg.effective_approval_policy(), ApprovalPolicy::OnFailure);
}

#[test]
fn allowed_low_level_policy_values_cap_effective_policy() {
    let cfg = cfg(r#"
sandbox_mode = "danger-full-access"
approval_policy = "never"

[requirements]
allowed_sandbox_modes = ["read-only"]
allowed_approval_policies = ["on-request"]
"#);
    assert_eq!(cfg.effective_sandbox_mode(), SandboxMode::ReadOnly);
    assert_eq!(cfg.effective_approval_policy(), ApprovalPolicy::OnRequest);
}

#[test]
fn allowed_permission_profiles_cap_effective_profile() {
    let cfg = cfg(r#"
permission_profile = ":danger-full-access"

[requirements]
allowed_permission_profiles = [":workspace"]
"#);
    assert_eq!(
        cfg.effective_permission_profile(),
        PermissionProfile::Workspace
    );
    assert_eq!(cfg.effective_access_mode(), Some(AccessMode::Approve));
}
