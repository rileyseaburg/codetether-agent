//! Invoke a tool while preserving bus event order and catalog guard lifetime.
use super::*;
impl McpServer {
    /// Handle call tool request
    pub(super) async fn handle_call_tool(
        &self,
        params: Option<Value>,
    ) -> Result<Value, JsonRpcError> {
        let params: CallToolParams = if let Some(p) = params {
            serde_json::from_value(p).map_err(|e| JsonRpcError::invalid_params(e.to_string()))?
        } else {
            return Err(JsonRpcError::invalid_params("Missing params"));
        };

        // Publish ToolRequest to the agent bus (picked up by S3 sink)
        let request_id = uuid::Uuid::new_v4().to_string();
        let bus_handle = self.agent_bus.as_ref().map(|bus| bus.handle("mcp-server"));
        if let Some(ref bh) = bus_handle {
            bh.send(
                format!("tools.{}", &params.name),
                BusMessage::ToolRequest {
                    request_id: request_id.clone(),
                    agent_id: "mcp-server".into(),
                    tool_name: params.name.clone(),
                    arguments: params.arguments.clone(),
                    step: 0,
                },
            );
        }

        let tools = self.catalog.tools().read().await;
        let handler = tools
            .get(&params.name)
            .ok_or_else(|| JsonRpcError::method_not_found(&params.name))?;

        let (result_value, output_text, success) =
            codetether_mcp_core::server_protocol::normalize_tool_result(handler(params.arguments))?;

        // Publish ToolResponse to the agent bus
        if let Some(ref bh) = bus_handle {
            bh.send(
                format!("tools.{}", &params.name),
                BusMessage::ToolResponse {
                    request_id,
                    agent_id: "mcp-server".into(),
                    tool_name: params.name,
                    result: super::super::bus_payload::tool_response(&output_text),
                    success,
                    step: 0,
                },
            );
        }

        Ok(result_value)
    }
}
