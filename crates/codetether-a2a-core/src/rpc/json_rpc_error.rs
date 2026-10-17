use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// JsonRpcError data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::JsonRpcError { code: 0, message: String::new(), data: None };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
