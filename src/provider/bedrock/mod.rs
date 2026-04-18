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
//! let region = AwsCredentials::detect_region().unwrap_or_else(|| "us-west-2".into());
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
pub mod auth;
pub mod body;
pub mod convert;
pub mod discovery;
pub mod estimates;
pub mod eventstream;
pub mod response;
pub mod retry;
pub mod sigv4;
pub mod stream;

pub use aliases::resolve_model_id;
pub use auth::{AwsCredentials, BedrockAuth};
pub use body::build_converse_body;
pub use convert::{convert_messages, convert_tools};
pub use estimates::{estimate_context_window, estimate_max_output};
pub use response::{BedrockError, parse_converse_response};

use crate::provider::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk};
use crate::util;
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use std::fmt;

/// Default AWS region when none is configured via env or config file.
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
/// let p = BedrockProvider::with_credentials(creds, "us-west-2".into()).unwrap();
/// assert_eq!(p.region(), "us-west-2");
/// ```
#[derive(Clone)]
pub struct BedrockProvider {
    pub(crate) client: Client,
    pub(crate) auth: BedrockAuth,
    pub(crate) region: String,
}

impl fmt::Debug for BedrockProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    #[allow(dead_code)]
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
            client: Client::new(),
            auth: BedrockAuth::BearerToken(api_key),
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
    /// let p = BedrockProvider::with_credentials(creds, "us-west-2".into()).unwrap();
    /// assert_eq!(p.region(), "us-west-2");
    /// ```
    pub fn with_credentials(credentials: AwsCredentials, region: String) -> Result<Self> {
        tracing::debug!(
            provider = "bedrock",
            region = %region,
            auth = "sigv4",
            "Creating Bedrock provider with AWS credentials"
        );
        Ok(Self {
            client: Client::new(),
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
            BedrockAuth::BearerToken(key) => {
                if key.is_empty() {
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

#[async_trait]
impl Provider for BedrockProvider {
    fn name(&self) -> &str {
        "bedrock"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        self.validate_auth()?;
        self.discover_models().await
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let model_id = Self::resolve_model_id(&request.model);

        tracing::debug!(
            provider = "bedrock",
            model = %model_id,
            original_model = %request.model,
            message_count = request.messages.len(),
            tool_count = request.tools.len(),
            "Starting Bedrock Converse request"
        );

        self.validate_auth()?;

        let body = self.build_converse_body(&request, model_id);

        // Keep the runtime URL readable; the SigV4 signer canonicalizes path
        // segments so model suffixes like `:0` are encoded exactly once when
        // constructing the signature.
        let url = format!("{}/model/{}/converse", self.base_url(), model_id);
        tracing::debug!("Bedrock request URL: {}", url);

        let body_bytes = serde_json::to_vec(&body)?;
        let policy = retry::RetryPolicy::default();

        for attempt in 1..=policy.max_attempts {
            let response = self
                .send_request("POST", &url, Some(&body_bytes), "bedrock")
                .await?;

            let status = response.status();
            let text = response
                .text()
                .await
                .context("Failed to read Bedrock response")?;

            if status.is_success() {
                return parse_converse_response(&text);
            }

            let retryable =
                retry::should_retry_status(status.as_u16()) && attempt < policy.max_attempts;
            if retryable {
                let sleep = policy.delay_for(attempt);
                tracing::warn!(
                    provider = "bedrock",
                    status = %status,
                    attempt,
                    sleep_ms = sleep.as_millis() as u64,
                    "Retrying Bedrock request after transient error"
                );
                tokio::time::sleep(sleep).await;
                continue;
            }

            if let Ok(err) = serde_json::from_str::<BedrockError>(&text) {
                anyhow::bail!("Bedrock API error ({}): {}", status, err.message);
            }
            anyhow::bail!(
                "Bedrock API error: {} {}",
                status,
                util::truncate_bytes_safe(&text, 500)
            );
        }

        unreachable!("retry loop exits via return or bail!");
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        let model_id = Self::resolve_model_id(&request.model);
        self.validate_auth()?;

        let body = self.build_converse_body(&request, model_id);
        let body_bytes = serde_json::to_vec(&body)?;
        self.converse_stream(model_id, body_bytes).await
    }
}

#[cfg(test)]
mod tests;
