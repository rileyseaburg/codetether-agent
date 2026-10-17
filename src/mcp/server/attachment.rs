//! Application registry and live bus attachment.
use super::*;
impl McpServer {
    /// Attach the full CodeTether tool registry to the MCP server.
    ///
    /// All tools from the registry will be exposed as MCP tools, replacing
    /// the hardcoded basic tool set. Call before [`Self::run`].
    pub fn with_tool_registry(mut self, registry: Arc<ToolRegistry>) -> Self {
        self.tool_registry = Some(registry);
        self
    }

    /// Attach a local agent bus for publishing tool calls to the S3 sink.
    ///
    /// Call this *before* [`Self::run`] so every tool invocation gets
    /// recorded as a `ToolRequest` + `ToolResponse` on the bus.
    pub fn with_agent_bus(mut self, bus: Arc<AgentBus>) -> Self {
        self.agent_bus = Some(bus);
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
        let bridge = BusBridge::with_auth(bus_url, auth_token).spawn();
        self.bus = Some(Arc::clone(&bridge));
        self.register_bus_tools(Arc::clone(&bridge)).await;
        self.register_bus_resources(Arc::clone(&bridge)).await;
        self
    }

    /// Register bus-specific MCP tools.
    pub(super) async fn register_bus_tools(&self, bridge: Arc<BusBridge>) {
        bus::register_tools(self, bridge).await;
    }

    /// Register bus-specific MCP resources.
    pub(super) async fn register_bus_resources(&self, bridge: Arc<BusBridge>) {
        bus::register_resources(self, bridge).await;
    }
}
