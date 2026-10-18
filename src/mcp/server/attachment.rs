//! Application registry and live bus attachment.
use super::*;
impl McpServer {
    /// Attach the full CodeTether tool registry to the MCP server.
    ///
    /// All tools from the registry will be exposed as MCP tools, replacing
    /// the hardcoded basic tool set. Call before [`Self::run`].
    pub fn with_tool_registry(mut self, registry: Arc<ToolRegistry>) -> Self {
        self.inner.tool_registry = Some(registry);
        self
    }

    /// Attach a local agent bus for publishing tool calls to the S3 sink.
    ///
    /// Call this *before* [`Self::run`] so every tool invocation gets
    /// recorded as a `ToolRequest` + `ToolResponse` on the bus.
    pub fn with_agent_bus(mut self, bus: Arc<AgentBus>) -> Self {
        self.inner.agent_bus = Some(bus);
        self
    }

    /// Attach a bus bridge and register bus-aware tools + resources.
    ///
    /// Call this *before* [`Self::run`] to enable live bus monitoring.
    pub async fn with_bus(self, bus_url: String) -> Self {
        self.with_bus_auth(bus_url, None).await
    }

    /// Attach a bus bridge with optional authentication.
    pub async fn with_bus_auth(mut self, bus_url: String, auth_token: Option<String>) -> Self {
        codetether_mcp_bus::attach(
            &mut self.inner.bus,
            &self.inner.catalog,
            bus_url,
            auth_token,
        )
        .await;
        self
    }
}
