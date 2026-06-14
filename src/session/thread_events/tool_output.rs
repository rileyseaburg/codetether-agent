//! Incremental tool output mapping.

use serde_json::json;

use crate::session::thread_store::ThreadEvent;

use super::mapper::ThreadEventMapper;

impl ThreadEventMapper {
    pub(super) fn tool_output_chunk(
        &mut self,
        tool_call_id: &str,
        name: &str,
        stream: &str,
        chunk: &str,
    ) -> ThreadEvent {
        let item_id = self.open_tool_item(tool_call_id);
        self.event(
            "tool.output_chunk",
            json!({
                "item_id": item_id,
                "tool_call_id": tool_call_id,
                "name": name,
                "stream": stream,
                "chunk": chunk,
            }),
        )
    }
}
