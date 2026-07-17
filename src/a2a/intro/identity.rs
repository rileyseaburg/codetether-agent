//! Restart-safe identity keys for introduction deduplication.

use sha2::{Digest, Sha256};

pub(super) fn key(endpoint: &str, peer_token: Option<&str>) -> String {
    let endpoint = endpoint.trim_end_matches('/');
    match peer_token {
        Some(token) => format!(
            "{endpoint}#{}:{}",
            digest(crate::a2a::collaboration_token::local()),
            digest(token)
        ),
        None => endpoint.to_string(),
    }
}

fn digest(token: &str) -> String {
    hex::encode(Sha256::digest(token.as_bytes()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn capability_distinguishes_recycled_endpoints_without_leaking() {
        let first = super::key("http://127.0.0.1:4000", Some("first-secret"));
        let second = super::key("http://127.0.0.1:4000", Some("second-secret"));
        assert_ne!(first, second);
        assert!(!first.contains("first-secret"));
    }
}
