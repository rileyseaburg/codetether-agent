//! Effective trust and tool policy status derived from loaded config.

use super::{
    AccessMode, ApprovalPolicy, Config, PermissionProfile, ProjectTrustLevel, SandboxMode,
};
use serde::{Deserialize, Serialize};

/// Effective trust, approval, sandbox, and permission profile state.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrustPolicyStatus {
    /// Effective project-local trust level.
    pub trust_level: ProjectTrustLevel,
    /// Effective named access mode when low-level policy values match one.
    pub access_mode: Option<AccessMode>,
    /// Effective tool approval policy.
    pub approval_policy: ApprovalPolicy,
    /// Effective filesystem sandbox mode.
    pub sandbox_mode: SandboxMode,
    /// Effective permission profile.
    pub permission_profile: PermissionProfile,
    /// Convenience flag for consumers that only need trusted/untrusted.
    pub project_trusted: bool,
}

impl TrustPolicyStatus {
    /// Build status from a loaded [`Config`].
    pub fn from_config(config: &Config) -> Self {
        Self {
            trust_level: config.effective_trust_level(),
            access_mode: config.effective_access_mode(),
            approval_policy: config.effective_approval_policy(),
            sandbox_mode: config.effective_sandbox_mode(),
            permission_profile: config.effective_permission_profile(),
            project_trusted: config.is_project_trusted(),
        }
    }
}

#[cfg(test)]
#[path = "trust_status_tests.rs"]
mod tests;
