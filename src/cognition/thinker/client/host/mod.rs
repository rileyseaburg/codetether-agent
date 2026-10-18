//! Application credentials and registry lookup for shared thinker services.
mod build;
use super::super::{ThinkerConfig, ThinkerOutput};
use crate::provider::bedrock::BedrockProvider;
use anyhow::Result;
use std::sync::Arc;
/// Application credential and registry hooks for the shared thinker service.
pub struct Native;
impl codetether_thinker_service::Host for Native {
    type Bedrock = Arc<BedrockProvider>;
    fn bedrock(config: &ThinkerConfig) -> Result<Self::Bedrock> {
        build::bedrock(config)
    }
    async fn registry(config: &ThinkerConfig, system: &str, user: &str) -> Result<ThinkerOutput> {
        super::super::provider::think(config, system, user).await
    }
    async fn send(provider: &Self::Bedrock, url: &str, bytes: &[u8]) -> Result<reqwest::Response> {
        provider.send_converse_request(url, bytes).await
    }
}
