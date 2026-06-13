//! Short-term Bedrock bearer-token generation from AWS credentials.
//!
//! Implements the same algorithm as AWS's official
//! `aws-bedrock-token-generator`: a SigV4-presigned
//! `Action=CallWithBearerToken` URL, base64-encoded with the
//! `bedrock-api-key-` prefix. The resulting token works anywhere a
//! Bedrock API key is accepted (e.g. `AWS_BEARER_TOKEN_BEDROCK`).
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::provider::bedrock::{AwsCredentials, token_gen};
//!
//! let creds = AwsCredentials {
//!     access_key_id: "AKIA...".into(),
//!     secret_access_key: "secret".into(),
//!     session_token: None,
//! };
//! let token = token_gen::generate_bearer_token(&creds, "us-east-1", 43200);
//! assert!(token.starts_with("bedrock-api-key-"));
//! ```

mod presign;
#[cfg(test)]
mod tests;

use super::auth::AwsCredentials;
use base64::Engine;

/// Default token lifetime (12 hours), matching AWS's generator.
pub const DEFAULT_EXPIRES_SECS: u64 = 43200;

/// Generate a short-term Bedrock bearer token (`bedrock-api-key-...`).
///
/// Token validity is `min(expires_secs, credential lifetime)`; tokens
/// signed with STS session credentials die when the session does.
pub fn generate_bearer_token(creds: &AwsCredentials, region: &str, expires_secs: u64) -> String {
    generate_at(creds, region, expires_secs, chrono::Utc::now())
}

/// Deterministic variant used by tests; signs at the provided instant.
pub fn generate_at(
    creds: &AwsCredentials,
    region: &str,
    expires_secs: u64,
    now: chrono::DateTime<chrono::Utc>,
) -> String {
    let query = presign::presigned_query(creds, region, expires_secs, now);
    let url = format!("bedrock.amazonaws.com/?{query}&Version=1");
    let encoded = base64::engine::general_purpose::STANDARD.encode(url.as_bytes());
    format!("bedrock-api-key-{encoded}")
}
