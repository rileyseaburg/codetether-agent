//! tests/tampered manifest fails verification for verified plugin registration.
use super::*;

#[test]
fn tampered_manifest_fails_verification() {
    let key = SigningKey::with_key(b"test-secret-key-for-signing".to_vec());
    let hash = hash_bytes(b"print('hello')");
    let sig = key.sign("my-plugin", "1.0.0", &hash);

    let manifest = PluginManifest {
        id: "my-plugin".to_string(),
        name: "My Plugin".to_string(),
        version: "1.0.1".to_string(), // tampered version
        content_hash: hash,
        signed_by: "test".to_string(),
        signature: sig,
        capabilities: vec![],
        timeout_secs: 30,
    };

    assert!(!key.verify(&manifest));
}
