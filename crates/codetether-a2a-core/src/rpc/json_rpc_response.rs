use super::*;

/// JSON-RPC 2.0 Response
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JsonRpcResponse data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::JsonRpcResponse { jsonrpc: String::new(), id: serde_json::Value::Null, result: None, error: None };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}
