//! Cached native provider registry bindings for shared swarm setup.
use super::tui_bridge::Observer;
use crate::provider::ProviderRegistry;
use codetether_swarm_tool::{provider_setup, runtime::SelectedProvider};

pub(super) async fn load(
    requested: Option<&str>,
    observer: &mut Observer,
) -> anyhow::Result<Result<SelectedProvider, crate::tool::ToolResult>> {
    provider_setup::load(requested, observer, ProviderRegistry::shared_from_vault()).await
}
