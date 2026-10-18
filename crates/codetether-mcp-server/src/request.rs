//! Request routing and JSON-RPC response construction.
use crate::Host;
use codetether_mcp_core::types::{JsonRpcRequest, JsonRpcResponse};
use serde_json::json;
/// Route one request and preserve its response identifier.
/// # Arguments
/// * `host` - Current server catalog and application invocation policy.
/// * `request` - Incoming JSON-RPC request.
/// # Returns
/// A success or error response with the original identifier.
/// # Examples
/// ```no_run
/// async fn ping(host: &impl codetether_mcp_server::Host) {
///     let request = codetether_mcp_core::types::JsonRpcRequest::new(1_i64, "ping", None);
///     let _response = codetether_mcp_server::request(host, request).await;
/// }
/// ```
pub async fn request(host: &impl Host, request: JsonRpcRequest) -> JsonRpcResponse {
    tracing::debug!(target: "codetether_agent::mcp::server",
        "Handling request: {} (id: {:?})",
        request.method, request.id
    );

    let result = match request.method.as_str() {
        "initialize" => codetether_mcp_core::server_protocol::initialize(
            request.params,
            host.server_info(),
            || host.approval_capability(),
        ),
        "initialized" => Ok(json!({})),
        "ping" => Ok(json!({})),
        "tools/list" => host.catalog().handle_list_tools(request.params).await,
        "tools/call" => host.call_tool(request.params).await,
        "resources/list" => host.catalog().handle_list_resources(request.params).await,
        "resources/read" => host.catalog().handle_read_resource(request.params).await,
        "prompts/list" => codetether_mcp_core::server_protocol::list_prompts(),
        "prompts/get" => codetether_mcp_core::server_protocol::get_prompt(request.params),
        method => host.approval(method, request.params).await,
    };

    match result {
        Ok(value) => JsonRpcResponse::success(request.id, value),
        Err(error) => JsonRpcResponse::error(request.id, error),
    }
}
