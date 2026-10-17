//! Private service-account, claim, and cache state.
use super::*;
/// Cached OAuth2 access token with expiration tracking
pub(crate) struct CachedToken {
    pub(crate) token: String,
    pub(crate) expires_at: std::time::Instant,
}
/// GCP service account key (parsed from JSON)
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ServiceAccountKey {
    pub(crate) client_email: String,
    pub(crate) private_key: String,
    pub(crate) token_uri: Option<String>,
    pub(crate) project_id: Option<String>,
}
/// JWT claims for GCP service account auth
#[derive(serde::Serialize)]
pub(crate) struct JwtClaims {
    pub(crate) iss: String,
    pub(crate) scope: String,
    pub(crate) aud: String,
    pub(crate) iat: u64,
    pub(crate) exp: u64,
}
