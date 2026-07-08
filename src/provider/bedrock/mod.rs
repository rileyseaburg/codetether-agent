//! Amazon Bedrock provider for the Converse API.
//!
//! Supports both AWS IAM SigV4 credentials and opaque bearer tokens (e.g.
//! from an API Gateway fronting Bedrock). Dispatches chat completions, tool
//! calls, and model discovery via the `bedrock-runtime` and `bedrock`
//! management APIs.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! use codetether_agent::provider::Provider;
//! use codetether_agent::provider::bedrock::{AwsCredentials, BedrockProvider};
//!
//! let creds = AwsCredentials::from_environment().expect("creds");
//! let region = AwsCredentials::detect_region().unwrap_or_else(|| "us-east-1".into());
//! let provider = BedrockProvider::with_credentials(creds, region).unwrap();
//! let models = provider.list_models().await.unwrap();
//! assert!(!models.is_empty());
//! # });
//! ```
//!
//! # Architecture
//!
//! - [`auth`] — credential loading, SigV4/bearer auth mode enum
//! - [`aliases`] — short-name to full model ID mapping
//! - [`sigv4`] — SigV4 signing + HTTP dispatch
//! - [`estimates`] — context-window / max-output heuristics
//! - [`convert`] — [`Message`] → Converse API JSON translation
//! - [`body`] — Converse request body builder
//! - [`response`] — Converse response parser
//! - [`discovery`] — dynamic model list via management APIs

pub mod aliases;
pub mod aliases_openai;
pub mod auth;
pub mod body;
pub mod convert;
pub mod discovery;
pub mod empty_guard;
pub mod estimates;
pub mod eventstream;
mod exports;
mod invoke;
pub mod output_budget;
mod provider_impl;
pub mod reasoning;
pub mod reasoning_audit;
pub mod response;
pub mod retry;
pub mod runtime_config;
pub mod sigv4;
pub mod sso_refresh;
pub mod stream;
pub mod token_gen;

pub use exports::*;

use crate::provider::CompletionRequest;
use {anyhow::Result, reqwest::Client};

/// Default AWS region when none is configured.
pub const DEFAULT_REGION: &str = "us-east-1";

/// Amazon Bedrock provider implementation.
///
/// Clone-able and cheap to copy (wraps an [`Arc`]-backed [`reqwest::Client`]).
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

impl BedrockProvider {
    /// Create a bearer-token provider in the default region.
    ///
    /// # Errors
    ///
    /// Currently infallible, but returns [`Result`] for API symmetry with
    /// [`BedrockProvider::with_credentials`] and future validation needs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::provider::bedrock::BedrockProvider;
    /// let p = BedrockProvider::new("token-abc".into()).unwrap();
    /// assert_eq!(p.region(), "us-east-1");
    /// ```
    pub fn new(api_key: String) -> Result<Self> {
        Self::with_region(api_key, DEFAULT_REGION.to_string())
    }

    /// Create a bearer-token provider with an explicit region.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::provider::bedrock::BedrockProvider;
    /// let p = BedrockProvider::with_region("token".into(), "eu-west-1".into()).unwrap();
    /// assert_eq!(p.region(), "eu-west-1");
    /// ```
    pub fn with_region(api_key: String, region: String) -> Result<Self> {
        tracing::debug!(
            provider = "bedrock",
            region = %region,
            auth = "bearer_token",
            "Creating Bedrock provider"
        );
        Ok(Self {
            client: crate::provider::shared_http::shared_client().clone(),
            auth: BedrockAuth::bearer(api_key),
            region,
        })
    }

    /// Create a SigV4 provider from AWS IAM credentials.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use codetether_agent::provider::bedrock::{AwsCredentials, BedrockProvider};
    /// let creds = AwsCredentials::from_environment().unwrap();
    /// let p = BedrockProvider::with_credentials(creds, "us-east-1".into()).unwrap();
    /// assert_eq!(p.region(), "us-east-1");
    /// ```
    pub fn with_credentials(credentials: AwsCredentials, region: String) -> Result<Self> {
        tracing::debug!(
            provider = "bedrock",
            region = %region,
            auth = "sigv4",
            "Creating Bedrock provider with AWS credentials"
        );
        Ok(Self {
            client: crate::provider::shared_http::shared_client().clone(),
            auth: BedrockAuth::SigV4(credentials),
            region,
        })
    }

    /// Return the configured AWS region.
    pub fn region(&self) -> &str {
        &self.region
    }

    /// Resolve a short model alias to a full Bedrock model ID.
    ///
    /// See [`aliases::resolve_model_id`] for details.
    pub fn resolve_model_id(model: &str) -> &str {
        aliases::resolve_model_id(model)
    }

    /// Build a Converse request body. See [`body::build_converse_body`].
    pub fn build_converse_body(
        &self,
        request: &CompletionRequest,
        model_id: &str,
    ) -> serde_json::Value {
        body::build_converse_body(request, model_id)
    }

    /// Validate that credentials/token are non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`anyhow::Error`] if the bearer token is empty or AWS keys
    /// are incomplete.
    pub(crate) fn validate_auth(&self) -> Result<()> {
        match &self.auth {
            BedrockAuth::BearerToken(cell) => {
                if cell.read().is_empty() {
                    anyhow::bail!("Bedrock API key is empty");
                }
            }
            BedrockAuth::SigV4(creds) => {
                if creds.access_key_id.is_empty() || creds.secret_access_key.is_empty() {
                    anyhow::bail!("AWS credentials are incomplete");
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
