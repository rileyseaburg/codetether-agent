//! Native factories and runtime policy for the shared application registry.
use super::{ToolRegistry, catalog, search_router};
use crate::provider::{Provider, ProviderRegistry};
use std::sync::Arc;

/// Native application bindings for the public tool registry alias.
pub struct Native;
impl codetether_application_registry::Application for Native {
    type Providers = ProviderRegistry;
    fn populate(registry: &mut ToolRegistry, provider: Option<(Arc<dyn Provider>, String)>) {
        let provider_backed = provider.is_some();
        codetether_tool_registry::catalog::populate(
            &mut catalog::Native::new(registry, provider),
            provider_backed,
            cfg!(feature = "tetherscript"),
        );
    }
    fn register_search(registry: &mut ToolRegistry, providers: Arc<ProviderRegistry>) {
        registry.register(Arc::new(search_router::SearchTool::new(providers)));
    }
    fn register_provider_search(registry: &mut ToolRegistry, provider: Arc<dyn Provider>) {
        let mut providers = ProviderRegistry::new();
        providers.register(Arc::clone(&provider));
        registry.register(Arc::new(search_router::SearchTool::new(Arc::new(
            providers,
        ))));
    }
    async fn evaluate(tool: &str, args: &serde_json::Value) -> Option<super::ToolResult> {
        crate::runtime_policy::evaluate_tool_invocation(tool, args).await
    }
}
