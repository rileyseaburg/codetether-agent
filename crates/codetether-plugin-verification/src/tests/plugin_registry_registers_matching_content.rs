//! tests/plugin registry registers matching content for verified plugin registration.
use super::*;

#[tokio::test]
async fn plugin_registry_registers_matching_content() {
    let key = SigningKey::with_key(b"test-key".to_vec());
    let manifest = signed_manifest(&key, "good-plugin", b"content");
    let registry = PluginRegistry::new(key);

    registry
        .register_bytes(manifest, b"content")
        .await
        .expect("matching content registers");
    assert!(registry.is_verified("good-plugin").await);
}
