use super::{ApprovalPolicy, PermissionProfile, SandboxMode};

impl PermissionProfile {
    pub fn implied_sandbox_mode(self) -> Option<SandboxMode> {
        match self {
            Self::ReadOnly => Some(SandboxMode::ReadOnly),
            Self::Workspace => Some(SandboxMode::WorkspaceWrite),
            Self::DangerFullAccess => Some(SandboxMode::DangerFullAccess),
            Self::Conservative | Self::Disabled => None,
        }
    }

    pub fn implied_approval_policy(self) -> Option<ApprovalPolicy> {
        match self {
            Self::ReadOnly => Some(ApprovalPolicy::OnRequest),
            Self::Workspace => Some(ApprovalPolicy::OnFailure),
            Self::DangerFullAccess => Some(ApprovalPolicy::Never),
            Self::Conservative | Self::Disabled => None,
        }
    }
}
