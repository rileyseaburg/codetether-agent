//! tests/plugin registry requires content by default for verified plugin registration.
use super::*;

#[tokio::test]
async fn plugin_registry_requires_content_by_default() {
    let key = SigningKey::with_key(b"test-key".to_vec());
    let manifest = signed_manifest(&key, "needs-content", b"content");
    let registry = PluginRegistry::new(key);

    assert!(registry.register(manifest).await.is_err());
}
