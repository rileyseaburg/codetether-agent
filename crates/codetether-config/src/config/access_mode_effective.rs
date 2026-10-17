use super::{AccessMode, ApprovalPolicy, PermissionProfile, SandboxMode};

impl AccessMode {
    pub fn from_effective(
        sandbox: SandboxMode,
        approval: ApprovalPolicy,
        profile: PermissionProfile,
    ) -> Option<Self> {
        match (sandbox, approval, profile) {
            (SandboxMode::ReadOnly, ApprovalPolicy::OnRequest, PermissionProfile::Conservative)
            | (SandboxMode::ReadOnly, ApprovalPolicy::OnRequest, PermissionProfile::ReadOnly) => {
                Some(Self::Ask)
            }
            (
                SandboxMode::WorkspaceWrite,
                ApprovalPolicy::OnFailure,
                PermissionProfile::Conservative | PermissionProfile::Workspace,
            ) => Some(Self::Approve),
            (
                SandboxMode::DangerFullAccess,
                ApprovalPolicy::Never,
                PermissionProfile::Conservative | PermissionProfile::DangerFullAccess,
            ) => Some(Self::Full),
            _ => None,
        }
    }
}
