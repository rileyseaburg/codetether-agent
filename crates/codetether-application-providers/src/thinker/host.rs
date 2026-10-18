//! Bind thinker service calls to cached providers and signed Bedrock transport.
use super::{ApplicationHost, Registry, bedrock};
use anyhow::Result;
use codetether_cognition_core::{ThinkerConfig, ThinkerOutput};
use codetether_provider_bedrock::provider::BedrockProvider;
use std::sync::Arc;

impl<H: Registry> codetether_thinker_service::Host for ApplicationHost<H> {
    type Bedrock = Arc<BedrockProvider>;
    fn bedrock(config: &ThinkerConfig) -> Result<Self::Bedrock> {
        bedrock::build(config)
    }
    async fn registry(config: &ThinkerConfig, system: &str, user: &str) -> Result<ThinkerOutput> {
        H::think(config, system, user).await
    }
    async fn send(provider: &Self::Bedrock, url: &str, bytes: &[u8]) -> Result<reqwest::Response> {
        provider.send_converse_request(url, bytes).await
    }
}
