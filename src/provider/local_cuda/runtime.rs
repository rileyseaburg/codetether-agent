//! Bind local CUDA provider execution to the existing thinker client.
use crate::cognition::{ThinkerClient, ThinkerConfig, ThinkerOutput};
/// Native thinker backend used by the local CUDA provider.
pub struct NativeRuntime(ThinkerClient);
impl codetether_provider_local_cuda::Runtime for NativeRuntime {
    fn new(config: ThinkerConfig) -> anyhow::Result<Self> {
        ThinkerClient::new(config).map(Self)
    }
    async fn think_as_provider(
        &self,
        provider: &str,
        system: &str,
        user: &str,
    ) -> anyhow::Result<ThinkerOutput> {
        self.0.think_as_provider(provider, system, user).await
    }
}
