//! SigV4 URL canonicalization, signing, and HTTP dispatch.
//!
//! [SigningClient] borrows the caller's HTTP client, authentication mode, and
//! region. [canonicalize_url] exposes the URL shape used by the signature.
use crate::{AwsCredentials, BedrockAuth};
use anyhow::{Context, Result};
use reqwest::Client;
mod authorization;
mod canonical;
mod crypto;
mod dispatch;
mod headers;
mod send;
pub use canonical::{CanonicalUrl, canonicalize_url};
pub use crypto::{hmac_sha256, sha256_hex};
/// Borrowed configuration for SigV4-signed HTTP calls.
///
/// # Examples
/// ```no_run
/// let client = reqwest::Client::new();
/// let auth = codetether_bedrock_auth::BedrockAuth::bearer("token".into());
/// let signer = codetether_bedrock_auth::sigv4::SigningClient::new(&client,&auth,"us-east-1");
/// ```
pub struct SigningClient<'a> {
    client: &'a Client,
    auth: &'a BedrockAuth,
    region: &'a str,
}
impl<'a> SigningClient<'a> {
    /// Borrow the caller's HTTP client, authentication mode, and AWS region.
    pub fn new(client: &'a Client, auth: &'a BedrockAuth, region: &'a str) -> Self {
        Self {
            client,
            auth,
            region,
        }
    }
}
struct SignedHeaders {
    host: String,
    amz_date: String,
    payload_hash: String,
    authorization: String,
}
mod credentials;
#[cfg(test)]
mod tests;
