//! Delegate registry operations through the application's client policy.
use super::*;
impl codetether_mcp_core::registry::RegistryClient for McpClient {
    fn tools(&self) -> impl std::future::Future<Output = Vec<McpTool>> + Send {
        McpClient::tools(self)
    }
    fn close(&self) -> impl std::future::Future<Output = Result<()>> + Send {
        McpClient::close(self)
    }
    fn call_tool(
        &self,
        name: &str,
        arguments: Value,
    ) -> impl std::future::Future<Output = Result<CallToolResult>> + Send {
        McpClient::call_tool(self, name, arguments)
    }
}
