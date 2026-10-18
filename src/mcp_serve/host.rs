//! Native Vault and full-tool-registry constructors for MCP serving.
use codetether_agent::{
    provider::{Provider, ProviderRegistry},
    tool::ToolRegistry,
};
use std::sync::Arc;
pub(super) struct Native;
impl codetether_mcp_cli::serve::ProviderHost for Native {
    type Providers = ProviderRegistry;
    type Tools = ToolRegistry;
    async fn providers() -> anyhow::Result<ProviderRegistry> {
        ProviderRegistry::from_vault().await
    }
    fn get(providers: &ProviderRegistry, name: &str) -> Option<Arc<dyn Provider>> {
        providers.get(name)
    }
    fn provider_tools(provider: Arc<dyn Provider>, model: String) -> Arc<ToolRegistry> {
        Arc::new(ToolRegistry::with_provider(provider, model))
    }
    fn default_tools() -> Arc<ToolRegistry> {
        Arc::new(ToolRegistry::with_defaults())
    }
}
