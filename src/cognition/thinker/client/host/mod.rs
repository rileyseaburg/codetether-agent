//! Native model and provider bindings for the shared thinker client.
mod build;
mod inference;
use super::super::{CandleThinker, ThinkerConfig, ThinkerOutput};
use crate::provider::bedrock::BedrockProvider;
use anyhow::Result;
use codetether_cognition_core::thinker::client::Host;
use std::sync::{Arc, Mutex};
/// Application backend hooks retained by the public thinker-client alias.
pub struct Native;
impl Host for Native {
    type Native = Arc<Mutex<CandleThinker>>;
    type Bedrock = Arc<BedrockProvider>;
    fn native(config: &ThinkerConfig) -> Result<Self::Native> {
        build::native(config)
    }
    fn bedrock(config: &ThinkerConfig) -> Result<Self::Bedrock> {
        build::bedrock(config)
    }
    async fn http(
        config: &ThinkerConfig,
        http: &reqwest::Client,
        system: &str,
        user: &str,
    ) -> Result<ThinkerOutput> {
        inference::http(config, http, system, user).await
    }
    async fn registry(config: &ThinkerConfig, system: &str, user: &str) -> Result<ThinkerOutput> {
        inference::registry(config, system, user).await
    }
    async fn bedrock_think(
        config: &ThinkerConfig,
        provider: &Self::Bedrock,
        system: &str,
        user: &str,
    ) -> Result<ThinkerOutput> {
        inference::bedrock(config, provider, system, user).await
    }
    async fn native_think(
        runtime: &Self::Native,
        provider: &str,
        system: &str,
        user: &str,
    ) -> Result<ThinkerOutput> {
        inference::native(runtime, provider, system, user).await
    }
}
