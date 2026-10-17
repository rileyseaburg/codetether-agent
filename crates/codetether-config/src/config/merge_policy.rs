use crate::config::Config;

impl Config {
    pub(super) fn merge_codex_policy(&mut self, other: &Self) {
        if other.access_mode.is_some() {
            self.access_mode = other.access_mode;
        }
        if other.sandbox_mode.is_some() {
            self.sandbox_mode = other.sandbox_mode;
        }
        if other.approval_policy.is_some() {
            self.approval_policy = other.approval_policy;
        }
        if other.trust_level.is_some() {
            self.trust_level = other.trust_level;
        }
        if other.permission_profile.is_some() {
            self.permission_profile
                .clone_from(&other.permission_profile);
        }
        if !other.requirements.is_empty() {
            self.requirements.clone_from(&other.requirements);
        }
    }
}
