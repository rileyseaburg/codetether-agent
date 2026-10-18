//! Cached native provider registry bindings for shared swarm setup.
use codetether_native_providers::ProviderRegistry;
use codetether_swarm_monitor::Observer;
use codetether_swarm_tool::{provider_setup, runtime::SelectedProvider};

pub(super) async fn load(
    requested: Option<&str>,
    observer: &mut Observer,
) -> anyhow::Result<Result<SelectedProvider, codetether_tool_core::ToolResult>> {
    provider_setup::load(requested, observer, ProviderRegistry::shared_from_vault()).await
}
