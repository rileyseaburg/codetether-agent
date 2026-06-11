use super::{format_status, set_status, trust_status_badge_span};
use crate::config::{
    AccessMode, ApprovalPolicy, PermissionProfile, ProjectTrustLevel, SandboxMode,
    TrustPolicyStatus,
};

fn status(project_trusted: bool) -> TrustPolicyStatus {
    TrustPolicyStatus {
        trust_level: if project_trusted {
            ProjectTrustLevel::Trusted
        } else {
            ProjectTrustLevel::Untrusted
        },
        approval_policy: ApprovalPolicy::OnRequest,
        access_mode: Some(AccessMode::Ask),
        sandbox_mode: SandboxMode::WorkspaceWrite,
        permission_profile: PermissionProfile::Conservative,
        project_trusted,
    }
}

#[test]
fn trust_status_formats_settings_text() {
    let text = format_status(&status(false));

    assert!(text.contains("Trust: untrusted"));
    assert!(text.contains("Access: ask"));
    assert!(text.contains("Approval: on-request"));
    assert!(text.contains("Sandbox: workspace-write"));
    assert!(text.contains("Profile: conservative"));
}

#[test]
fn trust_status_badge_uses_loaded_status() {
    set_status(status(true));

    let badge = trust_status_badge_span().expect("badge should be available");

    assert!(badge.content.contains("TRUST trusted"));
    assert!(badge.content.contains("ACCESS ask"));
    assert!(badge.content.contains("APPROVAL on-request"));
    assert!(badge.content.contains("SANDBOX workspace-write"));
}
