//! Shared cognition-client backend for the local CUDA provider.
use codetether_cognition_core::{
    ThinkerConfig, ThinkerOutput,
    thinker::client::{Client, Host},
};

impl<H: Host> crate::Runtime for Client<H>
where
    Client<H>: Send + Sync + 'static,
{
    fn new(config: ThinkerConfig) -> anyhow::Result<Self> {
        Client::new(config)
    }

    async fn think_as_provider(
        &self,
        provider: &str,
        system: &str,
        user: &str,
    ) -> anyhow::Result<ThinkerOutput> {
        Client::think_as_provider(self, provider, system, user).await
    }
}
