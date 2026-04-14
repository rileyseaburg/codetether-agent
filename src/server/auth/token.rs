use super::JwtClaims;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};

/// Parse a JWT token and extract claims from the payload without verifying
/// the signature.
///
/// This helper is only suitable for attaching informational claims that come
/// from a token that has already been authenticated by some other mechanism.
/// Do not treat a successful decode as proof that the token is trustworthy.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::server::auth::extract_unverified_jwt_claims;
///
/// let token = "aaa.eyJ0b3BpY3MiOlsiYWdlbnQuYWxwaGEiXSwic3ViIjoid29ya2VyLTEiLCJzY29wZXMiOlsiYnVzOnJlYWQiXX0.ccc";
/// let claims = extract_unverified_jwt_claims(token).expect("claims should decode");
///
/// assert_eq!(claims.subject.as_deref(), Some("worker-1"));
/// assert_eq!(claims.topics, vec!["agent.alpha"]);
/// ```
pub fn extract_unverified_jwt_claims(token: &str) -> Option<JwtClaims> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return None;
    }
    let payload = URL_SAFE_NO_PAD.decode(parts[1]).ok()?;
    // SECURITY: This only decodes the payload. Signature verification must
    // happen before calling this helper.
    serde_json::from_slice(&payload).ok()
}
