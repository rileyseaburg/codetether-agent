//! Native provider/tool and MCP transport bindings.
use codetether_bus_core::AgentBus;
use codetether_provider_core::Provider;
use std::{future::Future, sync::Arc};
/// Provider discovery and registry construction used by MCP serving.
/// # Implementors
/// Applications retain their Vault provider constructors and full tool registry.
pub trait ProviderHost {
    /// Discovered provider registry, kept alive during selection and tool construction.
    type Providers;
    /// Tool registry attached to the server.
    type Tools;
    /// Load providers using the existing native Vault path.
    /// # Errors
    /// Returns provider discovery errors, which serving logs before using defaults.
    fn providers() -> impl Future<Output = anyhow::Result<Self::Providers>>;
    /// Look up an existing provider by name.
    fn get(providers: &Self::Providers, name: &str) -> Option<Arc<dyn Provider>>;
    /// Build tools for the selected provider/model.
    fn provider_tools(provider: Arc<dyn Provider>, model: String) -> Arc<Self::Tools>;
    /// Build the default tools when provider discovery cannot select a provider.
    fn default_tools() -> Arc<Self::Tools>;
}
/// Native stdio server assembly and execution.
/// # Implementors
/// Keep transport construction, bus authentication and serving in the native adapter.
pub trait Host: ProviderHost {
    /// Configured native MCP server.
    type Server;
    /// Construct a stdio server with tools and a local event bus.
    fn server(tools: Arc<Self::Tools>, bus: Arc<AgentBus>) -> Self::Server;
    /// Attach a remote bus bridge and its optional authentication token.
    fn attach(
        server: Self::Server,
        url: String,
        token: Option<String>,
    ) -> impl Future<Output = Self::Server>;
    /// Execute the MCP server loop.
    /// # Errors
    /// Returns transport or protocol execution errors.
    fn run(server: Self::Server) -> impl Future<Output = anyhow::Result<()>>;
}
