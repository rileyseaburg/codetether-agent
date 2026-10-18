//! Resolve current providers and build the native autonomous tool registry.
use super::{Application, Ephemeral, ToolRegistry};
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_spawn::EphemeralResources;
use codetether_provider_core::Provider;
use std::{path::Path, sync::Arc};
#[async_trait]
impl<T: Application, const MAX_STEPS: usize> EphemeralResources for Ephemeral<T, MAX_STEPS> {
    type Registry = ToolRegistry<T>;
    async fn resolve(model: &str) -> Result<(Arc<dyn Provider>, String)> {
        let providers = crate::provider_cache::get_registry().await?;
        providers.resolve_model(model)
    }
    fn registry(
        read_only: bool,
        verification: bool,
        workspace: &Path,
        provider: Arc<dyn Provider>,
        model: String,
    ) -> Arc<ToolRegistry<T>> {
        codetether_native_swarm::direct::registry::standard::<T>(
            read_only,
            verification,
            workspace,
            provider,
            model,
        )
    }
}
