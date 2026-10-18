//! MCP server transport, catalog, identity, and attached services.
use codetether_bus_core::AgentBus;
use codetether_mcp_core::{server_catalog::ServerCatalog, types::ServerInfo};
use codetether_mcp_transport::Transport;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Server state with application-supplied bus-bridge and tool-registry types.
/// # Examples
/// ```no_run
/// # fn example(transport: std::sync::Arc<dyn codetether_mcp_transport::Transport>) {
/// let state = codetether_mcp_server::ServerState::<(), ()>::new(transport, "1.0");
/// assert_eq!(state.server_info.name, "codetether");
/// assert!(state.tool_registry.is_none());
/// # }
/// ```
pub struct ServerState<B, R> {
    /// Transport carrying MCP requests and responses.
    pub transport: Arc<dyn Transport>,
    /// Registered tool, prompt, and resource handlers.
    pub catalog: ServerCatalog,
    /// Whether the MCP initialization notification has been received.
    pub initialized: RwLock<bool>,
    /// Application server name and version advertised to clients.
    pub server_info: ServerInfo,
    /// Optional live event bridge.
    pub bus: Option<Arc<B>>,
    /// Optional local bus used to record tool calls.
    pub agent_bus: Option<Arc<AgentBus>>,
    /// Optional application registry exposed through MCP.
    pub tool_registry: Option<Arc<R>>,
}

impl<B, R> ServerState<B, R> {
    /// Create uninitialized state with an empty catalog and no attached services.
    /// # Arguments
    /// * `transport` — Existing MCP transport, retained without replacement.
    /// * `version` — Host application version advertised in server identity.
    /// # Returns
    /// State ready for application registry and bus attachment.
    /// # Examples
    /// See [ServerState] for construction with a caller-provided transport.
    pub fn new(transport: Arc<dyn Transport>, version: &str) -> Self {
        Self {
            transport,
            catalog: ServerCatalog::new(),
            initialized: RwLock::new(false),
            server_info: ServerInfo {
                name: "codetether".to_string(),
                version: version.to_string(),
            },
            bus: None,
            agent_bus: None,
            tool_registry: None,
        }
    }
}
