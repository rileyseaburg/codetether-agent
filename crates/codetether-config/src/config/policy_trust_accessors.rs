use crate::config::{Config, ProjectTrustLevel};

impl Config {
    /// Return the effective project trust level.
    pub fn effective_trust_level(&self) -> ProjectTrustLevel {
        self.trust_level.unwrap_or_default()
    }

    /// Return whether project-local configuration has been trusted.
    pub fn is_project_trusted(&self) -> bool {
        matches!(self.effective_trust_level(), ProjectTrustLevel::Trusted)
    }
}
