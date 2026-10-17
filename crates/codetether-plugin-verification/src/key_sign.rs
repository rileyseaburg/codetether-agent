//! key sign for verified plugin registration.
use super::*;
impl SigningKey {
    /// Sign a manifest payload: `id|version|content_hash`.
    pub fn sign(&self, id: &str, version: &str, content_hash: &str) -> String {
        use hmac::{Hmac, KeyInit, Mac};
        type HmacSha256 = Hmac<Sha256>;

        let payload = format!("{}|{}|{}", id, version, content_hash);
        let mut mac = HmacSha256::new_from_slice(&self.key).expect("HMAC can take key of any size");
        mac.update(payload.as_bytes());
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }
}
