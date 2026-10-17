use super::{AccessMode, Config};

impl Config {
    /// Apply a transient access-mode override from CLI or server request input.
    pub fn apply_access_mode_override(&mut self, access_mode: Option<AccessMode>) {
        if let Some(access_mode) = access_mode {
            self.access_mode = Some(access_mode);
            self.sandbox_mode = None;
            self.approval_policy = None;
            self.permission_profile = None;
        }
    }
}
