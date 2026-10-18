//! Bedrock authenticated HTTP dispatch and management model discovery.
//!
//! [BedrockClient] borrows an existing HTTP client, auth state, and region.
//! It does not load secrets or own token refresh policy.
//! [provider::BedrockProvider] owns provider state and integrates construction,
//! validation, protocol routing and credential recovery.
use anyhow::Result;
use codetether_bedrock_auth::BedrockAuth;
use reqwest::Client;
mod discovery;
mod dispatch;
mod signed;
mod urls;
/// Borrowed Bedrock HTTP client and credential context.
///
/// # Examples
/// ```no_run
/// let http = reqwest::Client::new();
/// let auth = codetether_bedrock_auth::BedrockAuth::bearer("token".into());
/// let client = codetether_provider_bedrock::BedrockClient::new(&http,&auth,"us-east-1");
/// assert!(client.base_url().contains("us-east-1"));
/// ```
pub struct BedrockClient<'a> {
    client: &'a Client,
    auth: &'a BedrockAuth,
    region: &'a str,
}
impl<'a> BedrockClient<'a> {
    /// Borrow the configured client, authentication state, and AWS region.
    ///
    /// # Arguments
    /// * `client` - HTTP client with the caller's connection/time-out settings.
    /// * `auth` - IAM credentials or shared bearer token.
    /// * `region` - Region used for URLs and signing.
    ///
    /// # Returns
    /// A client whose lifetime is bounded by the borrowed configuration.
    ///
    /// # Examples
    /// See [BedrockClient].
    pub fn new(client: &'a Client, auth: &'a BedrockAuth, region: &'a str) -> Self {
        Self {
            client,
            auth,
            region,
        }
    }
}

mod converse;
pub mod invoke;
pub mod recovery;
pub mod retry;
mod runtime_url;
mod stream;

impl BedrockClient<'_> {
    /// Return the borrowed AWS region.
    pub fn region(&self) -> &str {
        self.region
    }
}

/// Owning Bedrock provider, credentials, protocol facades and recovery integration.
pub mod provider;
