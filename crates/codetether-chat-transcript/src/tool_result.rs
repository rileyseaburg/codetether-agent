//! Recover tool names and preserve result success and payload limits.
use crate::PartProjection;
use codetether_chat_model::{ChatMessage, MessageType, payload};
use codetether_util::preview_chars::truncate_preview;
impl PartProjection<'_> {
    pub(crate) fn tool_result(&mut self, tool_call_id: &str, content: &str) {
        self.flush();
        let name = self
            .names
            .get(tool_call_id)
            .cloned()
            .unwrap_or_else(|| "tool".to_string());
        let success = !content.trim_start().starts_with("Error:");
        self.messages.push(ChatMessage::new(
            MessageType::ToolResult {
                name: name.clone(),
                output: payload::tool_output(content),
                success,
                duration_ms: None,
            },
            format!("{name}: {}", truncate_preview(content, 600)),
        ));
    }
}
