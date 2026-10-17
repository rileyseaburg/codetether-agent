use super::{AccessMode, ApprovalPolicy, PermissionProfile, SandboxMode};
use serde::{Deserialize, Serialize};

/// Policy constraints that cap user-selected access and permission settings.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyRequirements {
    /// Allowed shorthand access modes. Empty means unrestricted.
    #[serde(default)]
    pub allowed_access_modes: Vec<AccessMode>,
    /// Allowed filesystem sandbox modes. Empty means unrestricted.
    #[serde(default)]
    pub allowed_sandbox_modes: Vec<SandboxMode>,
    /// Allowed approval policies. Empty means unrestricted.
    #[serde(default)]
    pub allowed_approval_policies: Vec<ApprovalPolicy>,
    /// Allowed permission profiles. Empty means unrestricted.
    #[serde(default)]
    pub allowed_permission_profiles: Vec<PermissionProfile>,
}

impl PolicyRequirements {
    pub fn is_empty(&self) -> bool {
        self.allowed_access_modes.is_empty()
            && self.allowed_sandbox_modes.is_empty()
            && self.allowed_approval_policies.is_empty()
            && self.allowed_permission_profiles.is_empty()
    }
}
