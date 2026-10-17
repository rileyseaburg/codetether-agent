use super::*;

/// JSON-RPC 2.0 Request
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JsonRpcRequest data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::JsonRpcRequest { jsonrpc: String::new(), id: serde_json::Value::Null, method: String::new(), params: serde_json::Value::Null };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: serde_json::Value,
    pub method: String,
    #[serde(default)]
    pub params: serde_json::Value,
}
