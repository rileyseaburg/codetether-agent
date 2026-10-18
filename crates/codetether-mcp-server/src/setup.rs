//! Registry selection and registration reporting before serving requests.
use codetether_mcp_core::server_catalog::ServerCatalog;
use std::future::Future;

/// Supplies the configured registry and native registration hooks.
/// # Implementors
/// Servers expose their existing catalog and optional application tool registry.
pub trait Host: Sync {
    /// Registry whose tools are exposed when configured.
    type Registry: Sync;
    /// Catalog that receives registrations and supplies the final tool count.
    fn catalog(&self) -> &ServerCatalog;
    /// Optional configured registry, preferred over the fallback catalog.
    fn registry(&self) -> Option<&Self::Registry>;
    /// Register every eligible tool from the configured registry.
    fn register_registry(&self, registry: &Self::Registry) -> impl Future<Output = ()> + Send;
    /// Register the built-in fallback tools when no registry is configured.
    fn register_fallback(&self) -> impl Future<Output = ()> + Send;
}

/// Register configured or fallback tools and report the resulting catalog size.
/// # Arguments
/// * `host` — Server supplying its catalog and registration operations.
/// # Returns
/// Completes after registration and the matching count log have finished.
/// # Examples
/// ```no_run
/// # async fn example(host: &impl codetether_mcp_server::setup::Host) {
/// codetether_mcp_server::setup::run(host).await;
/// # }
/// ```
pub async fn run(host: &impl Host) {
    if let Some(registry) = host.registry() {
        host.register_registry(registry).await;
        tracing::info!(target: "codetether_agent::mcp::server",
            "Registered {} MCP tools from ToolRegistry",
            host.catalog().tools().read().await.len()
        );
        return;
    }

    host.register_fallback().await;
    tracing::info!(target: "codetether_agent::mcp::server",
        "Registered {} MCP tools (fallback)",
        host.catalog().tools().read().await.len()
    );
}
