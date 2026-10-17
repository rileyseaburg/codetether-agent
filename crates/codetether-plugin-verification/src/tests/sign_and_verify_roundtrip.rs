//! tests/sign and verify roundtrip for verified plugin registration.
use super::*;

#[test]
fn sign_and_verify_roundtrip() {
    let key = SigningKey::with_key(b"test-secret-key-for-signing".to_vec());
    let hash = hash_bytes(b"print('hello')");
    let sig = key.sign("my-plugin", "1.0.0", &hash);

    let manifest = PluginManifest {
        id: "my-plugin".to_string(),
        name: "My Plugin".to_string(),
        version: "1.0.0".to_string(),
        content_hash: hash,
        signed_by: "test".to_string(),
        signature: sig,
        capabilities: vec!["fs:read".to_string()],
        timeout_secs: 30,
    };

    assert!(key.verify(&manifest));
}
