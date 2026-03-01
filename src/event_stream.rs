//! Event streaming module for real-time event propagation
//!
//! Provides event streaming capabilities for the CodeTether agent.

pub mod s3_sink;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

/// Chat event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChatEvent {
    /// User message
    UserMessage { content: String, timestamp: i64 },
    /// Assistant message
    AssistantMessage { content: String, timestamp: i64 },
    /// Tool call
    ToolCall {
        tool_name: String,
        arguments: serde_json::Value,
        timestamp: i64,
    },
    /// Tool result
    ToolResult {
        tool_name: String,
        result: String,
        success: bool,
        timestamp: i64,
    },
}

impl ChatEvent {
    /// Create a tool result event
    pub fn tool_result(
        workspace: std::path::PathBuf,
        _session_id: String,
        tool_name: &str,
        success: bool,
        _duration_ms: u64,
        result: &str,
        _message_count: u64,
    ) -> Self {
        let _ = workspace; // Suppress unused warning
        Self::ToolResult {
            tool_name: tool_name.to_string(),
            result: result.to_string(),
            success,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

/// Event types that can be streamed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[allow(dead_code)]
pub enum Event {
    /// Tool execution started
    ToolStarted {
        tool_name: String,
        request_id: String,
    },
    /// Tool execution completed
    ToolCompleted {
        tool_name: String,
        request_id: String,
        success: bool,
    },
    /// Agent message
    AgentMessage { content: String },
    /// Error occurred
    Error { message: String },
    /// Chat event
    Chat(ChatEvent),
}

/// Event stream handle
#[allow(dead_code)]
pub struct EventStream {
    sender: broadcast::Sender<Event>,
}

impl EventStream {
    #[allow(dead_code)]
    /// Create a new event stream
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(256);
        Self { sender }
    }

    #[allow(dead_code)]
    /// Subscribe to events
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }

    #[allow(dead_code)]
    /// Send an event
    pub fn send(&self, event: Event) -> Result<()> {
        self.sender.send(event)?;
        Ok(())
    }
}

impl Default for EventStream {
    fn default() -> Self {
        Self::new()
    }
}
