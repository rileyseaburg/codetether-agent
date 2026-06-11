use crate::config::{
    AccessMode, ApprovalPolicy, Config, PermissionProfile, PermissionProfileConfig, SandboxMode,
};

impl Config {
    pub(super) fn raw_permission_profile(&self) -> PermissionProfile {
        self.permission_profile
            .as_ref()
            .map(PermissionProfileConfig::profile_type)
            .or_else(|| self.access_mode.map(AccessMode::permission_profile))
            .unwrap_or_default()
    }

    pub(super) fn raw_sandbox_mode(&self) -> SandboxMode {
        self.sandbox_mode
            .or_else(|| self.access_mode.map(AccessMode::sandbox_mode))
            .or_else(|| self.raw_permission_profile().implied_sandbox_mode())
            .unwrap_or_default()
    }

    pub(super) fn raw_approval_policy(&self) -> ApprovalPolicy {
        self.approval_policy
            .or_else(|| self.access_mode.map(AccessMode::approval_policy))
            .or_else(|| self.raw_permission_profile().implied_approval_policy())
            .unwrap_or_default()
    }

    pub(super) fn raw_access_mode(&self) -> Option<AccessMode> {
        self.access_mode.or_else(|| {
            AccessMode::from_effective(
                self.raw_sandbox_mode(),
                self.raw_approval_policy(),
                self.raw_permission_profile(),
            )
        })
    }
}
