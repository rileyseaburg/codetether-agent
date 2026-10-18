//! Native provider search registration and nested invocation authorization.
use super::{Application, Catalog, Provider, ProviderRegistry, ToolRegistry};
use codetether_native_search::tool::SearchTool;
use std::sync::Arc;
impl<C: Catalog> codetether_application_registry::Application for Application<C> {
    type Providers = ProviderRegistry;
    fn populate(registry: &mut ToolRegistry<Self>, provider: Option<(Arc<dyn Provider>, String)>) {
        C::populate(registry, provider);
    }
    fn register_search(registry: &mut ToolRegistry<Self>, providers: Arc<ProviderRegistry>) {
        registry.register(Arc::new(SearchTool::new(providers)));
    }
    fn register_provider_search(registry: &mut ToolRegistry<Self>, provider: Arc<dyn Provider>) {
        let mut providers = ProviderRegistry::new();
        providers.register(Arc::clone(&provider));
        registry.register(Arc::new(SearchTool::new(Arc::new(providers))));
    }
    async fn evaluate(
        tool: &str,
        args: &serde_json::Value,
    ) -> Option<codetether_tool_core::ToolResult> {
        codetether_native_runtime_policy::evaluate_tool_invocation(tool, args).await
    }
}
