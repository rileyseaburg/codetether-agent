//! registry from env for verified plugin registration.
use super::*;
impl PluginRegistry {
    pub fn from_env() -> Self {
        Self::new(SigningKey::shared())
    }
}
