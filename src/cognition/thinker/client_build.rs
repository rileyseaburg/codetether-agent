//! Backend construction from a [`ThinkerConfig`](super::ThinkerConfig).

use anyhow::{Context, Result, anyhow};
use reqwest::Client;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::client_backend::ThinkerClientBackend;
use super::{CandleThinker, ThinkerBackend, ThinkerConfig};
use crate::provider::bedrock::{AwsCredentials, BedrockProvider};

/// Build the runtime backend selected by `config`.
///
/// # Errors
///
/// Returns an error when the HTTP client cannot be built, the Candle runtime
/// fails to load, or Bedrock credentials are unavailable.
pub(super) fn build(config: &ThinkerConfig) -> Result<ThinkerClientBackend> {
    match config.backend {
        ThinkerBackend::OpenAICompat => http_backend(config),
        ThinkerBackend::Registry => Ok(ThinkerClientBackend::Registry),
        ThinkerBackend::Candle => Ok(ThinkerClientBackend::Candle {
            runtime: Arc::new(Mutex::new(CandleThinker::new(config)?)),
        }),
        ThinkerBackend::Bedrock => bedrock_backend(config),
    }
}

fn http_backend(config: &ThinkerConfig) -> Result<ThinkerClientBackend> {
    let http = Client::builder()
        .timeout(Duration::from_millis(config.timeout_ms.max(1_000)))
        .build()
        .context("failed to build thinker HTTP client")?;
    Ok(ThinkerClientBackend::OpenAICompat { http })
}

fn bedrock_backend(config: &ThinkerConfig) -> Result<ThinkerClientBackend> {
    let creds = AwsCredentials::from_environment().ok_or_else(|| {
        anyhow!(
            "Bedrock thinker requires AWS credentials (AWS_ACCESS_KEY_ID/AWS_SECRET_ACCESS_KEY or ~/.aws/credentials)"
        )
    })?;
    let provider = BedrockProvider::with_credentials(creds, config.bedrock_region.clone())?;
    Ok(ThinkerClientBackend::Bedrock {
        provider: Arc::new(provider),
    })
}
