//! tests/plugin registry rejects bad signature for verified plugin registration.
use super::*;

#[tokio::test]
async fn plugin_registry_rejects_bad_signature() {
    let key = SigningKey::with_key(b"test-key".to_vec());
    let registry = PluginRegistry::new(key);

    let manifest = PluginManifest {
        id: "bad-plugin".to_string(),
        name: "Bad".to_string(),
        version: "0.1.0".to_string(),
        content_hash: "abc".to_string(),
        signed_by: "attacker".to_string(),
        signature: "definitely-wrong".to_string(),
        capabilities: vec![],
        timeout_secs: 10,
    };

    assert!(registry.register_bytes(manifest, b"content").await.is_err());
    assert!(!registry.is_verified("bad-plugin").await);
}
