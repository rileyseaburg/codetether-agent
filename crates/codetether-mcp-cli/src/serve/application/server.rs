//! Assemble stdio transport and preserve authenticated remote-bus attachment.
use super::ApplicationHost;
use crate::serve::Host;
use codetether_application_providers::{Application as ProviderApplication, ProviderRegistry};
use codetether_application_registry::{Application as ToolApplication, ToolRegistry};
use codetether_bus_core::AgentBus;
use codetether_mcp_server::{Application as ServerApplication, ApplicationServer};
use std::sync::Arc;
impl<P, T, S> Host for ApplicationHost<ProviderRegistry<P>, ToolRegistry<T>, ApplicationServer<S>>
where
    P: ProviderApplication,
    T: ToolApplication,
    S: ServerApplication<Registry = ToolRegistry<T>>,
{
    type Server = ApplicationServer<S>;
    fn server(tools: Arc<ToolRegistry<T>>, bus: Arc<AgentBus>) -> ApplicationServer<S> {
        ApplicationServer::new_stdio()
            .with_tool_registry(tools)
            .with_agent_bus(bus)
    }
    async fn attach(server: Self::Server, url: String, token: Option<String>) -> Self::Server {
        server.with_bus_auth(url, token).await
    }
    async fn run(server: Self::Server) -> anyhow::Result<()> {
        server.run().await
    }
}
