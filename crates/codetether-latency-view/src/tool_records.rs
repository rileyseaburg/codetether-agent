//! Timed tool records selected from the current chat buffer.
use crate::ToolLatency;
use codetether_chat_model::{ChatMessage, MessageType};

impl ToolLatency {
    /// Collect completed tool results that include measured durations.
    /// # Arguments
    /// * `messages` - Chat messages in display order.
    /// # Returns
    /// Timed tool records in the same order, retaining failure status.
    /// Untimed results and other message kinds are omitted.
    /// # Examples
    /// ```
    /// assert!(codetether_latency_view::ToolLatency::from_messages(&[]).is_empty());
    /// ```
    pub fn from_messages(messages: &[ChatMessage]) -> Vec<Self> {
        messages
            .iter()
            .filter_map(|message| match &message.message_type {
                MessageType::ToolResult {
                    name,
                    success,
                    duration_ms: Some(duration_ms),
                    ..
                } => Some(ToolLatency {
                    name: name.clone(),
                    success: *success,
                    duration_ms: *duration_ms,
                }),
                _ => None,
            })
            .collect()
    }
}
