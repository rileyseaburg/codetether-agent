//! Dispatch provider parts in their original order.
use crate::PartProjection;
use codetether_provider_core::ContentPart;
impl PartProjection<'_> {
    pub(crate) fn push(&mut self, part: &ContentPart) {
        match part {
            ContentPart::Text { text } => crate::buffer::push_line(&mut self.text_buffer, text),
            ContentPart::Thinking { text, .. } => self.thinking(text),
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } => self.tool_call(id, name, arguments),
            ContentPart::ToolResult {
                tool_call_id,
                content,
            } => self.tool_result(tool_call_id, content),
            ContentPart::Image { url, .. } => self.image(url),
            ContentPart::File { path, .. } => self.file(path),
        }
    }
}
