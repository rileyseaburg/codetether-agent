//! Events carried by the broadcast channel.

use super::ChatEvent;
use serde::{Deserialize, Serialize};

/// A tool-start, tool-completion, agent-message, error, or chat notification.
///
/// # Examples
/// ```
/// use codetether_event_stream::Event;
/// let event = Event::Error { message: "unavailable".into() };
/// assert!(matches!(event, Event::Error { .. }));
/// ```
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
