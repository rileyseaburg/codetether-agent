//! Application provider state and redacted diagnostics.
//!
//! [BedrockProvider] keeps the shared HTTP client, mutable auth, and region
//! used by the borrowed execution client and application recovery adapter.
use super::BedrockAuth;
use reqwest::Client;

/// Amazon Bedrock provider implementation.
///
/// Clone-able and cheap to copy (wraps an [`std::sync::Arc`]-backed [`reqwest::Client`]).
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::provider::bedrock::{AwsCredentials, BedrockProvider};
///
/// let creds = AwsCredentials::from_environment().unwrap();
/// let p = BedrockProvider::with_credentials(creds, "us-east-1".into()).unwrap();
/// assert_eq!(p.region(), "us-east-1");
/// ```
#[derive(Clone)]
pub struct BedrockProvider {
    pub(crate) client: Client,
    pub(crate) auth: BedrockAuth,
    pub(crate) region: String,
}
impl std::fmt::Debug for BedrockProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BedrockProvider")
            .field(
                "auth",
                &match &self.auth {
                    BedrockAuth::SigV4(_) => "SigV4",
                    BedrockAuth::BearerToken(_) => "BearerToken",
                },
            )
            .field("region", &self.region)
            .finish()
    }
}
