use super::*;
/// Build the initialization response after validating client parameters.
///
/// # Arguments
/// `params` is the wire request; `server_info` identifies the host;
/// `capability` lazily supplies application approval capabilities.
/// # Returns
/// The serialized initialization response.
/// # Errors
/// Invalid or absent parameters and response serialization errors.
///
/// # Examples
/// ```
/// use codetether_mcp_core::{server_protocol::initialize, types::ServerInfo};
/// let server = ServerInfo { name: "example".into(), version: "1".into() };
/// assert!(initialize(None, &server, || None).is_err());
/// ```
pub fn initialize(
    params: Option<Value>,
    server_info: &ServerInfo,
    capability: impl FnOnce() -> Option<Value>,
) -> Result<Value, JsonRpcError> {
    let _params: InitializeParams = if let Some(p) = params {
        serde_json::from_value(p).map_err(|e| JsonRpcError::invalid_params(e.to_string()))?
    } else {
        return Err(JsonRpcError::invalid_params("Missing params"));
    };

    let result = InitializeResult {
        protocol_version: PROTOCOL_VERSION.to_string(),
        capabilities: ServerCapabilities {
            tools: Some(ToolsCapability { list_changed: true }),
            resources: Some(ResourcesCapability {
                subscribe: false,
                list_changed: true,
            }),
            prompts: Some(PromptsCapability { list_changed: true }),
            logging: Some(LoggingCapability {}),
            experimental: capability(),
        },
        server_info: server_info.clone(),
        instructions: Some(
            "CodeTether is an AI coding agent with tools for file operations, \
                 command execution, code search, and autonomous task execution. \
                 Use the swarm tool for complex tasks requiring parallel execution, \
                 and ralph for PRD-driven development."
                .to_string(),
        ),
    };

    serde_json::to_value(result).map_err(|e| JsonRpcError::internal_error(e.to_string()))
}
