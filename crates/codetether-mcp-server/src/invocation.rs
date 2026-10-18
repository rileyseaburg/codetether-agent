//! Correlated tool execution while retaining the catalog read guard.
use codetether_bus_core::{AgentBus, BusMessage};
use codetether_mcp_core::{
    server_catalog::ServerCatalog,
    types::{CallToolParams, JsonRpcError},
};
use serde_json::Value;
/// Invoke a catalog tool and publish its correlated bus events.
/// # Arguments
/// * `catalog` - Catalog whose read guard remains held through execution.
/// * `bus` - Optional existing event bus.
/// * `params` - JSON-RPC tool call parameters.
/// # Returns
/// The normalized tool result for the wire response.
/// # Errors
/// Returns invalid-parameter, missing-handler or normalization errors.
/// # Examples
/// ```no_run
/// async fn call(catalog: &codetether_mcp_core::server_catalog::ServerCatalog) {
///     let result = codetether_mcp_server::invoke(catalog, None, None).await;
///     assert!(result.is_err());
/// }
/// ```
pub async fn invoke(
    catalog: &ServerCatalog,
    bus: Option<&AgentBus>,
    params: Option<Value>,
) -> Result<Value, JsonRpcError> {
    let params: CallToolParams = if let Some(p) = params {
        serde_json::from_value(p).map_err(|e| JsonRpcError::invalid_params(e.to_string()))?
    } else {
        return Err(JsonRpcError::invalid_params("Missing params"));
    };

    // Publish ToolRequest to the agent bus (picked up by S3 sink)
    let request_id = uuid::Uuid::new_v4().to_string();
    let bus_handle = bus.map(|bus| bus.handle("mcp-server"));
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

    let tools = catalog.tools().read().await;
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
                result: crate::payload::tool_response(&output_text),
                success,
                step: 0,
            },
        );
    }

    Ok(result_value)
}
