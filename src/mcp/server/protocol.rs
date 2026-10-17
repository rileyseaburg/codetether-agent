//! Forward wire operations to shared protocol and catalog services.
use super::*;
impl McpServer {
    /// Handle initialize request
    pub(super) async fn handle_initialize(
        &self,
        params: Option<Value>,
    ) -> Result<Value, JsonRpcError> {
        codetether_mcp_core::server_protocol::initialize(
            params,
            &self.server_info,
            super::super::approval::capability,
        )
    }

    /// Handle list tools request - reads from ToolMetadata registry
    pub(super) async fn handle_list_tools(
        &self,
        _params: Option<Value>,
    ) -> Result<Value, JsonRpcError> {
        self.catalog.handle_list_tools(_params).await
    }

    /// Handle list resources request — reads from registered resource metadata
    pub(super) async fn handle_list_resources(
        &self,
        _params: Option<Value>,
    ) -> Result<Value, JsonRpcError> {
        self.catalog.handle_list_resources(_params).await
    }

    /// Handle read resource request
    pub(super) async fn handle_read_resource(
        &self,
        params: Option<Value>,
    ) -> Result<Value, JsonRpcError> {
        self.catalog.handle_read_resource(params).await
    }

    /// Handle list prompts request
    pub(super) async fn handle_list_prompts(
        &self,
        _params: Option<Value>,
    ) -> Result<Value, JsonRpcError> {
        codetether_mcp_core::server_protocol::list_prompts()
    }

    /// Handle get prompt request
    pub(super) async fn handle_get_prompt(
        &self,
        params: Option<Value>,
    ) -> Result<Value, JsonRpcError> {
        codetether_mcp_core::server_protocol::get_prompt(params)
    }
}
