//! Built-in MCP commands using application-selected registries and servers.
//!
//! [`ApplicationHost`] constructs a fresh local server for each operation and
//! retains the supplied server's tool registration and authorization policies.
use codetether_application_registry::{Application as ToolApplication, ToolRegistry};
use codetether_mcp_connections::ConnectionClient;
use codetether_mcp_core::types::{CallToolResult, ToolMetadata};
use codetether_mcp_server::{Application as ServerApplication, ApplicationServer};
use std::{marker::PhantomData, sync::Arc};

/// Bind built-in tool operations and the authorized external client.
///
/// # Examples
/// ```no_run
/// # fn example<T, S, C>() where T: codetether_application_registry::Application,
/// # S: codetether_mcp_server::Application<Registry = codetether_application_registry::ToolRegistry<T>>,
/// # C: codetether_mcp_connections::ConnectionClient {
/// type Bound<T, S, C> = codetether_mcp_cli::application::ApplicationHost<
///     codetether_application_registry::ToolRegistry<T>,
///     codetether_mcp_server::ApplicationServer<S>, C>;
/// fn requires_host<H: codetether_mcp_cli::Host>() {}
/// requires_host::<Bound<T, S, C>>();
/// # }
/// ```
pub struct ApplicationHost<T, S, C>(PhantomData<fn() -> (T, S, C)>);

#[async_trait::async_trait(?Send)]
impl<T, S, C> crate::Host for ApplicationHost<ToolRegistry<T>, ApplicationServer<S>, C>
where
    T: ToolApplication,
    S: ServerApplication<Registry = ToolRegistry<T>>,
    C: ConnectionClient,
{
    type Client = C;
    async fn builtin_tools() -> Vec<ToolMetadata> {
        let registry = Arc::new(ToolRegistry::<T>::with_defaults());
        let server = ApplicationServer::<S>::new_local().with_tool_registry(registry);
        server.setup_tools_public().await;
        server.get_all_tool_metadata().await
    }
    async fn builtin_call(
        tool: &str,
        arguments: serde_json::Value,
    ) -> anyhow::Result<CallToolResult> {
        let registry = Arc::new(ToolRegistry::<T>::with_defaults());
        let server = ApplicationServer::<S>::new_local().with_tool_registry(registry);
        server.setup_tools_public().await;
        server.call_tool_direct(tool, arguments).await
    }
}
