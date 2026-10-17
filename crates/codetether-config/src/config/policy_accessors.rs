use crate::config::{AccessMode, ApprovalPolicy, Config, PermissionProfile, SandboxMode};

impl Config {
    /// Return the effective sandbox mode, including the conservative default.
    pub fn effective_sandbox_mode(&self) -> SandboxMode {
        let raw = if let Some(mode) = self.requirement_access_mode() {
            mode.sandbox_mode()
        } else if self.sandbox_mode.is_none()
            && let Some(profile) = self.requirement_permission_profile()
        {
            profile
                .implied_sandbox_mode()
                .unwrap_or_else(|| self.raw_sandbox_mode())
        } else {
            self.raw_sandbox_mode()
        };
        self.requirements.sandbox_mode(raw)
    }

    /// Return the effective approval policy, including the conservative default.
    pub fn effective_approval_policy(&self) -> ApprovalPolicy {
        let raw = if let Some(mode) = self.requirement_access_mode() {
            mode.approval_policy()
        } else if self.approval_policy.is_none()
            && let Some(profile) = self.requirement_permission_profile()
        {
            profile
                .implied_approval_policy()
                .unwrap_or_else(|| self.raw_approval_policy())
        } else {
            self.raw_approval_policy()
        };
        self.requirements.approval_policy(raw)
    }

    /// Return the effective permission profile type.
    pub fn effective_permission_profile(&self) -> PermissionProfile {
        let raw = if let Some(mode) = self.requirement_access_mode() {
            mode.permission_profile()
        } else {
            self.raw_permission_profile()
        };
        self.requirements.permission_profile(raw)
    }

    /// Return the effective named access mode when policy values match one.
    pub fn effective_access_mode(&self) -> Option<AccessMode> {
        let mode = AccessMode::from_effective(
            self.effective_sandbox_mode(),
            self.effective_approval_policy(),
            self.effective_permission_profile(),
        )?;
        Some(self.requirements.access_mode(mode))
    }
}
