//! Bridge local tool calls to authorized external MCP connections.
//!
//! [McpBridgeTool] lists or calls remote tools and lists or reads resources.
//! [Host] preserves application policy; [Resources] extends the shared MCP
//! wrapper client. Each action retains its original connection and close order.
mod args;
mod call_tool;
mod connect;
mod execute;
mod host;
mod list_resources;
mod list_tools;
pub mod policy;
mod read_resource;
mod resources;
mod schema;
mod tool;
pub use host::Host;
pub use resources::Resources;
/// MCP bridge bound to the host's existing client and authorization hooks.
///
/// # Examples
/// ```rust
/// # fn example<H:codetether_mcp_bridge::Host>() {
/// let _tool=codetether_mcp_bridge::McpBridgeTool::<H>::new();
/// # }
/// ```
pub struct McpBridgeTool<H>(std::marker::PhantomData<H>);
impl<H: Host> McpBridgeTool<H> {
    /// Construct a bridge using the host's existing authorization hooks.
    ///
    /// # Examples
    /// ```rust
    /// # fn example<H:codetether_mcp_bridge::Host>() {
    /// let _tool=codetether_mcp_bridge::McpBridgeTool::<H>::new();
    /// # }
    /// ```
    pub fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<H: Host> Default for McpBridgeTool<H> {
    fn default() -> Self {
        Self::new()
    }
}
