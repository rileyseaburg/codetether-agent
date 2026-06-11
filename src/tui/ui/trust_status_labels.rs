//! Compact labels for trust policy status.

use crate::config::{
    AccessMode, ApprovalPolicy, PermissionProfile, SandboxMode, TrustPolicyStatus,
};

pub(super) fn trust(status: &TrustPolicyStatus) -> &'static str {
    if status.project_trusted {
        "trusted"
    } else {
        "untrusted"
    }
}

pub(super) fn approval(policy: ApprovalPolicy) -> &'static str {
    match policy {
        ApprovalPolicy::Untrusted => "untrusted",
        ApprovalPolicy::OnFailure => "on-failure",
        ApprovalPolicy::OnRequest => "on-request",
        ApprovalPolicy::Never => "never",
    }
}

pub(super) fn access(mode: Option<AccessMode>) -> &'static str {
    match mode {
        Some(AccessMode::Ask) => "ask",
        Some(AccessMode::Approve) => "approve",
        Some(AccessMode::Full) => "full",
        None => "custom",
    }
}

pub(super) fn sandbox(mode: SandboxMode) -> &'static str {
    match mode {
        SandboxMode::ReadOnly => "read-only",
        SandboxMode::WorkspaceWrite => "workspace-write",
        SandboxMode::DangerFullAccess => "danger-full-access",
    }
}

pub(super) fn profile(profile: PermissionProfile) -> &'static str {
    match profile {
        PermissionProfile::Conservative => "conservative",
        PermissionProfile::ReadOnly => ":read-only",
        PermissionProfile::Workspace => ":workspace",
        PermissionProfile::DangerFullAccess => ":danger-full-access",
        PermissionProfile::Disabled => "disabled",
    }
}
