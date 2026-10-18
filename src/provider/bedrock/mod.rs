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
//! # Ownership
//!
//! - State and construction live in the local adapter modules.
//! - `codetether-bedrock-auth` owns credentials, signing, and bearer tokens.
//! - `codetether-bedrock-protocol` owns request/response conversion and streams.
//! - `codetether-provider-bedrock` owns discovery and request execution.
//! - `codetether-bedrock-refresh` owns SSO exchange and secret persistence.
//! - The application supplies routing and credential-recovery integration.

pub mod aliases;
pub mod aliases_openai;
pub mod auth;
pub mod body;
mod client;
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

mod accessors;
mod construct;
mod state;
mod validation;
pub use state::BedrockProvider;

/// Default AWS region when none is configured.
pub use codetether_bedrock_auth::DEFAULT_REGION;
#[cfg(test)]
mod tests;
