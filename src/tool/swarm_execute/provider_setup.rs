//! Cached native provider registry bindings for shared swarm setup.
use super::tui_bridge::Observer;
use crate::provider::{Provider, ProviderRegistry};
use codetether_swarm_tool::{provider_setup, runtime::SelectedProvider};
use std::sync::Arc;

impl provider_setup::Registry for ProviderRegistry {
    fn list(&self) -> Vec<&str> {
        ProviderRegistry::list(self)
    }
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        ProviderRegistry::get(self, name)
    }
}

pub(super) async fn load(
    requested: Option<&str>,
    observer: &mut Observer,
) -> anyhow::Result<Result<SelectedProvider, crate::tool::ToolResult>> {
    provider_setup::load(requested, observer, ProviderRegistry::shared_from_vault()).await
}
