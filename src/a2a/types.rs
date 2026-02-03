//! A2A Protocol Types
//!
//! Types aligned with the A2A specification (a2a.json schema)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A2A Task States
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TaskState {
    Submitted,
    Working,
    Completed,
    Failed,
    Cancelled,
    InputRequired,
    Rejected,
    AuthRequired,
}

impl TaskState {
    /// Check if this is a terminal state
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            TaskState::Completed | TaskState::Failed | TaskState::Cancelled | TaskState::Rejected
        )
    }

    /// Check if this is an active state
    pub fn is_active(&self) -> bool {
        !self.is_terminal()
    }
}

/// An A2A Task
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub context_id: Option<String>,
    pub status: TaskStatus,
    #[serde(default)]
    pub artifacts: Vec<Artifact>,
    #[serde(default)]
    pub history: Vec<Message>,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Task status information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskStatus {
    pub state: TaskState,
    #[serde(default)]
    pub message: Option<Message>,
    pub timestamp: Option<String>,
}

/// An A2A Message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub message_id: String,
    pub role: MessageRole,
    pub parts: Vec<Part>,
    #[serde(default)]
    pub context_id: Option<String>,
    #[serde(default)]
    pub task_id: Option<String>,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Agent,
}

/// A part of a message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Part {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "file")]
    File { file: FileContent },
    #[serde(rename = "data")]
    Data { data: serde_json::Value },
}

/// File content (bytes or URI)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<String>, // Base64 encoded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// An artifact produced by a task
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artifact {
    pub artifact_id: String,
    pub parts: Vec<Part>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Agent Card - self-describing manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentCard {
    pub name: String,
    pub description: String,
    pub url: String,
    pub version: String,
    #[serde(default = "default_protocol_version")]
    pub protocol_version: String,
    pub capabilities: AgentCapabilities,
    pub skills: Vec<AgentSkill>,
    #[serde(default)]
    pub default_input_modes: Vec<String>,
    #[serde(default)]
    pub default_output_modes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<AgentProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
}

fn default_protocol_version() -> String {
    "0.3.0".to_string()
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentCapabilities {
    #[serde(default)]
    pub streaming: bool,
    #[serde(default)]
    pub push_notifications: bool,
    #[serde(default)]
    pub state_transition_history: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentSkill {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub examples: Vec<String>,
    #[serde(default)]
    pub input_modes: Vec<String>,
    #[serde(default)]
    pub output_modes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProvider {
    pub organization: String,
    pub url: String,
}

/// JSON-RPC 2.0 Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: serde_json::Value,
    pub method: String,
    #[serde(default)]
    pub params: serde_json::Value,
}

/// JSON-RPC 2.0 Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[allow(dead_code)]
impl JsonRpcError {
    /// Create a parse error (-32700)
    pub fn parse_error(msg: impl Into<String>) -> Self {
        Self {
            code: PARSE_ERROR,
            message: msg.into(),
            data: None,
        }
    }

    /// Create an invalid request error (-32600)
    pub fn invalid_request(msg: impl Into<String>) -> Self {
        Self {
            code: INVALID_REQUEST,
            message: msg.into(),
            data: None,
        }
    }

    /// Create a method not found error (-32601)
    pub fn method_not_found(method: &str) -> Self {
        Self {
            code: METHOD_NOT_FOUND,
            message: format!("Method not found: {}", method),
            data: None,
        }
    }

    /// Create an invalid params error (-32602)
    pub fn invalid_params(msg: impl Into<String>) -> Self {
        Self {
            code: INVALID_PARAMS,
            message: msg.into(),
            data: None,
        }
    }

    /// Create an internal error (-32603)
    pub fn internal_error(msg: impl Into<String>) -> Self {
        Self {
            code: INTERNAL_ERROR,
            message: msg.into(),
            data: None,
        }
    }
}

// Standard JSON-RPC error codes
#[allow(dead_code)]
pub const PARSE_ERROR: i32 = -32700;
#[allow(dead_code)]
pub const INVALID_REQUEST: i32 = -32600;
pub const METHOD_NOT_FOUND: i32 = -32601;
pub const INVALID_PARAMS: i32 = -32602;
pub const INTERNAL_ERROR: i32 = -32603;

// A2A-specific error codes
pub const TASK_NOT_FOUND: i32 = -32001;
pub const TASK_NOT_CANCELABLE: i32 = -32002;
#[allow(dead_code)]
pub const PUSH_NOT_SUPPORTED: i32 = -32003;
pub const UNSUPPORTED_OPERATION: i32 = -32004;
#[allow(dead_code)]
pub const CONTENT_TYPE_NOT_SUPPORTED: i32 = -32005;

/// Message send parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageSendParams {
    pub message: Message,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<MessageSendConfiguration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageSendConfiguration {
    #[serde(default)]
    pub accepted_output_modes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_notification_config: Option<PushNotificationConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushNotificationConfig {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Task query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskQueryParams {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_length: Option<usize>,
}
