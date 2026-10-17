//! tests/signed manifest for verified plugin registration.
use super::*;

pub(super) fn signed_manifest(key: &SigningKey, id: &str, content: &[u8]) -> PluginManifest {
    let hash = hash_bytes(content);
    PluginManifest {
        id: id.to_string(),
        name: id.to_string(),
        version: "1.0.0".to_string(),
        content_hash: hash.clone(),
        signed_by: "test".to_string(),
        signature: key.sign(id, "1.0.0", &hash),
        capabilities: vec!["fs:read".to_string()],
        timeout_secs: 30,
    }
}
