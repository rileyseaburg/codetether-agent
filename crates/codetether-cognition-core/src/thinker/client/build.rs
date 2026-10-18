use super::super::{ThinkerBackend, ThinkerConfig};
use super::{Host, backend::Backend};
use anyhow::{Context, Result};
use reqwest::Client;
use std::time::Duration;
pub(super) fn build<H: Host>(config: &ThinkerConfig) -> Result<Backend<H::Native, H::Bedrock>> {
    match config.backend {
        ThinkerBackend::OpenAICompat => http_backend(config),
        ThinkerBackend::Registry => Ok(Backend::Registry),
        ThinkerBackend::Candle => Ok(Backend::Candle {
            runtime: H::native(config)?,
        }),
        ThinkerBackend::Bedrock => Ok(Backend::Bedrock {
            provider: H::bedrock(config)?,
        }),
    }
}

fn http_backend<N, B>(config: &ThinkerConfig) -> Result<Backend<N, B>> {
    let http = Client::builder()
        .timeout(Duration::from_millis(config.timeout_ms.max(1_000)))
        .build()
        .context("failed to build thinker HTTP client")?;
    Ok(Backend::OpenAICompat { http })
}
