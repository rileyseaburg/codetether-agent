//! Conversation participant roles shared by provider adapters.

use serde::{Deserialize, Serialize};

/// Participant role in a conversation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// System prompt and session-wide instructions.
    System,
    /// Developer-provided contextual instructions in conversation history.
    Developer,
    /// End-user input.
    User,
    /// Model response.
    Assistant,
    /// Tool result returned to the model.
    Tool,
}
