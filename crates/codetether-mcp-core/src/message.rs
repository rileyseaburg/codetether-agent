//! Classify decoded JSON-RPC values as MCP requests, responses or notifications.
use crate::types::*;
use anyhow::Result;
use serde_json::Value;

/// Incoming MCP message (can be request, response, or notification)
#[derive(Debug, Clone)]
pub enum McpMessage {
    Request(JsonRpcRequest),
    Response(JsonRpcResponse),
    Notification(JsonRpcNotification),
}

impl McpMessage {
    /// Classify and decode a JSON-RPC request, response or notification.
    ///
    /// # Arguments
    /// * `value` — Parsed JSON-RPC message.
    /// # Returns
    /// The decoded message variant.
    /// # Errors
    /// Returns a decoding error when required wire fields are missing or invalid.
    /// # Examples
    /// ```
    /// let message = codetether_mcp_core::McpMessage::from_json(serde_json::json!({"jsonrpc":"2.0", "id":1, "method":"tools/list"})).unwrap();
    /// assert!(matches!(message, codetether_mcp_core::McpMessage::Request(_)));
    /// ```
    pub fn from_json(value: Value) -> Result<Self> {
        // Check if it has an id
        if value.get("id").is_some() {
            // Has id - could be request or response
            if value.get("method").is_some() {
                // Has method - it's a request
                let request: JsonRpcRequest = serde_json::from_value(value)?;
                Ok(McpMessage::Request(request))
            } else {
                // No method - it's a response
                let response: JsonRpcResponse = serde_json::from_value(value)?;
                Ok(McpMessage::Response(response))
            }
        } else {
            // No id - it's a notification
            let notification: JsonRpcNotification = serde_json::from_value(value)?;
            Ok(McpMessage::Notification(notification))
        }
    }
}
