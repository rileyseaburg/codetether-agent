//! Map [`SessionEvent`](crate::session::SessionEvent) variants.

use crate::session::SessionEvent;
use crate::session::thread_store::ThreadEvent;

use super::mapper::ThreadEventMapper;

impl ThreadEventMapper {
    /// Map one live session event into zero or more thread events.
    pub fn map_session_event(&mut self, event: &SessionEvent) -> Vec<ThreadEvent> {
        match event {
            SessionEvent::TextChunk(text) => self.text_chunk(text),
            SessionEvent::TextComplete(text) => self.text_completed(text),
            SessionEvent::ToolCallStart {
                tool_call_id,
                name,
                arguments,
            } => self.tool_start_events(tool_call_id, name, arguments),
            SessionEvent::ToolCallComplete {
                tool_call_id,
                name,
                output,
                success,
                duration_ms,
            } => self.tool_complete_events(tool_call_id, name, output, *success, *duration_ms),
            SessionEvent::ToolCallMetadata {
                tool_call_id,
                name,
                metadata,
            } => self.tool_metadata_events(tool_call_id, name, metadata.clone()),
            SessionEvent::ApprovalRequest(request) => vec![self.approval_requested(request)],
            SessionEvent::Done => vec![self.event("turn.done", serde_json::json!({}))],
            SessionEvent::Error(error) => vec![self.turn_failed(error)],
            _ => Vec::new(),
        }
    }
}
