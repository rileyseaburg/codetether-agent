//! tests/plugin registry rejects wrong content hash for verified plugin registration.
use super::*;

#[tokio::test]
async fn plugin_registry_rejects_wrong_content_hash() {
    let key = SigningKey::with_key(b"test-key".to_vec());
    let manifest = signed_manifest(&key, "mismatch-plugin", b"content");
    let registry = PluginRegistry::new(key);

    assert!(
        registry
            .register_bytes(manifest, b"tampered")
            .await
            .is_err()
    );
    assert!(!registry.is_verified("mismatch-plugin").await);
}
