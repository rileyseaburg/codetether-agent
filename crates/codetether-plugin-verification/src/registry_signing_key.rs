//! registry signing key for verified plugin registration.
use super::*;
impl PluginRegistry {
    /// Get a reference to the signing key (for creating manifests).
    pub fn signing_key(&self) -> &SigningKey {
        &self.signing_key
    }
}
