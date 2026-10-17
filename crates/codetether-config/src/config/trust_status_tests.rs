use super::TrustPolicyStatus;
use crate::config::{
    AccessMode, ApprovalPolicy, Config, PermissionProfile, PermissionProfileConfig,
    ProjectTrustLevel, SandboxMode,
};

#[test]
fn trust_policy_status_uses_effective_defaults() {
    let status = TrustPolicyStatus::from_config(&Config::default());

    assert_eq!(status.trust_level, ProjectTrustLevel::Untrusted);
    assert_eq!(status.access_mode, Some(AccessMode::Ask));
    assert_eq!(status.approval_policy, ApprovalPolicy::OnRequest);
    assert_eq!(status.sandbox_mode, SandboxMode::ReadOnly);
    assert_eq!(status.permission_profile, PermissionProfile::Conservative);
    assert!(!status.project_trusted);
}

#[test]
fn trust_policy_status_reflects_explicit_policy() {
    let config = Config {
        trust_level: Some(ProjectTrustLevel::Trusted),
        approval_policy: Some(ApprovalPolicy::Never),
        sandbox_mode: Some(SandboxMode::WorkspaceWrite),
        permission_profile: Some(PermissionProfileConfig::Named(PermissionProfile::Disabled)),
        ..Config::default()
    };

    let status = TrustPolicyStatus::from_config(&config);

    assert_eq!(status.trust_level, ProjectTrustLevel::Trusted);
    assert_eq!(status.access_mode, None);
    assert_eq!(status.approval_policy, ApprovalPolicy::Never);
    assert_eq!(status.sandbox_mode, SandboxMode::WorkspaceWrite);
    assert_eq!(status.permission_profile, PermissionProfile::Disabled);
    assert!(status.project_trusted);
}
