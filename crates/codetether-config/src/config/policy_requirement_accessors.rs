use crate::config::{AccessMode, Config, PermissionProfile};

impl Config {
    pub(super) fn requirement_access_mode(&self) -> Option<AccessMode> {
        if self.requirements.allowed_access_modes.is_empty() {
            return None;
        }
        self.raw_access_mode()
            .map(|mode| self.requirements.access_mode(mode))
    }

    pub(super) fn requirement_permission_profile(&self) -> Option<PermissionProfile> {
        if self.requirements.allowed_permission_profiles.is_empty() {
            return None;
        }
        Some(
            self.requirements
                .permission_profile(self.raw_permission_profile()),
        )
    }
}
