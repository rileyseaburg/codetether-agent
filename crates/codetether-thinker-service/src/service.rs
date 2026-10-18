//! Backend hook implementation for the shared thinker client.
use crate::{Host, ThinkerConfig, ThinkerOutput};
use anyhow::Result;
use codetether_cognition_native::CandleThinker;
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};
/// Backend services selected by the application's credential and registry host.
/// # Examples
/// ```no_run
/// use codetether_thinker_service::{Host, Service};
/// use codetether_cognition_core::thinker::{ThinkerConfig, client::Client};
/// fn build<H: Host>(config: ThinkerConfig) -> anyhow::Result<Client<Service<H>>> {
///     Client::new(config)
/// }
/// ```
pub struct Service<H: Host>(PhantomData<fn() -> H>);
impl<H: Host> codetether_cognition_core::thinker::client::Host for Service<H> {
    type Native = Arc<Mutex<CandleThinker>>;
    type Bedrock = H::Bedrock;
    fn native(config: &ThinkerConfig) -> Result<Self::Native> {
        crate::build::native(config)
    }
    fn bedrock(config: &ThinkerConfig) -> Result<Self::Bedrock> {
        H::bedrock(config)
    }
    async fn http(
        config: &ThinkerConfig,
        http: &reqwest::Client,
        system: &str,
        user: &str,
    ) -> Result<ThinkerOutput> {
        crate::openai_backend::think(config, http, system, user).await
    }
    async fn registry(config: &ThinkerConfig, system: &str, user: &str) -> Result<ThinkerOutput> {
        H::registry(config, system, user).await
    }
    async fn bedrock_think(
        config: &ThinkerConfig,
        provider: &Self::Bedrock,
        system: &str,
        user: &str,
    ) -> Result<ThinkerOutput> {
        crate::bedrock_backend::think::<H>(config, provider, system, user).await
    }
    async fn native_think(
        runtime: &Self::Native,
        provider: &str,
        system: &str,
        user: &str,
    ) -> Result<ThinkerOutput> {
        codetether_cognition_native::dispatch::think(runtime, provider, system, user).await
    }
}
