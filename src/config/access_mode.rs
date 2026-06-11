use super::{ApprovalPolicy, PermissionProfile, SandboxMode};
use serde::{Deserialize, Serialize};

/// User-facing access mode shorthand for sandbox and approval policy.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AccessMode {
    /// Ask before risky or mutating work, with read-only filesystem access.
    Ask,
    /// Run inside the workspace sandbox and ask only for escalation paths.
    Approve,
    /// Run without sandboxing or approval prompts.
    Full,
}

impl AccessMode {
    /// Return this mode's filesystem sandbox policy.
    pub fn sandbox_mode(self) -> SandboxMode {
        match self {
            Self::Ask => SandboxMode::ReadOnly,
            Self::Approve => SandboxMode::WorkspaceWrite,
            Self::Full => SandboxMode::DangerFullAccess,
        }
    }

    /// Return this mode's tool approval policy.
    pub fn approval_policy(self) -> ApprovalPolicy {
        match self {
            Self::Ask => ApprovalPolicy::OnRequest,
            Self::Approve => ApprovalPolicy::OnFailure,
            Self::Full => ApprovalPolicy::Never,
        }
    }

    /// Return this mode's permission profile.
    pub fn permission_profile(self) -> PermissionProfile {
        PermissionProfile::Conservative
    }
}
