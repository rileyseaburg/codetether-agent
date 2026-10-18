use super::super::super::{
    ThinkerOutput, bedrock_backend, candle_dispatch, openai_backend, provider,
};
use super::{CandleThinker, ThinkerConfig};
use crate::provider::bedrock::BedrockProvider;
use anyhow::Result;
use std::sync::{Arc, Mutex};
pub(super) async fn http(
    config: &ThinkerConfig,
    http: &reqwest::Client,
    system: &str,
    user: &str,
) -> Result<ThinkerOutput> {
    openai_backend::think(config, http, system, user).await
}
pub(super) async fn registry(
    config: &ThinkerConfig,
    system: &str,
    user: &str,
) -> Result<ThinkerOutput> {
    provider::think(config, system, user).await
}
pub(super) async fn bedrock(
    config: &ThinkerConfig,
    provider: &BedrockProvider,
    system: &str,
    user: &str,
) -> Result<ThinkerOutput> {
    bedrock_backend::think(config, provider, system, user).await
}
pub(super) async fn native(
    runtime: &Arc<Mutex<CandleThinker>>,
    provider: &str,
    system: &str,
    user: &str,
) -> Result<ThinkerOutput> {
    candle_dispatch::think(runtime, provider, system, user).await
}
