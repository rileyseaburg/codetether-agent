//! enabled policy implementation.
use super::*;

impl DelegationState {
    /// Whether CADMAS-CTX routing is active.
    pub fn enabled(&self) -> bool {
        env_enabled_override().unwrap_or(self.config.enabled)
    }
}
