//! key verify for verified plugin registration.
use super::*;
impl SigningKey {
    /// Verify a manifest signature.
    pub fn verify(&self, manifest: &PluginManifest) -> bool {
        let expected = self.sign(&manifest.id, &manifest.version, &manifest.content_hash);
        constant_time_eq(expected.as_bytes(), manifest.signature.as_bytes())
    }
}
