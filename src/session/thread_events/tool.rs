//! Tool lifecycle mapping.

use serde_json::json;

use crate::session::thread_store::ThreadEvent;

use super::mapper::ThreadEventMapper;

impl ThreadEventMapper {
    pub(super) fn tool_started(
        &mut self,
        tool_call_id: &str,
        name: &str,
        arguments: &str,
    ) -> ThreadEvent {
        let item_id = self.open_tool_item(tool_call_id);
        self.event(
            "tool.started",
            json!({
                "item_id": item_id,
                "tool_call_id": tool_call_id,
                "name": name,
                "arguments": arguments,
            }),
        )
    }

    pub(super) fn tool_completed(
        &mut self,
        tool_call_id: &str,
        name: &str,
        output: &str,
        success: bool,
        duration_ms: u64,
    ) -> ThreadEvent {
        let item_id = self.open_tool_item(tool_call_id);
        self.event(
            "tool.completed",
            json!({
                "item_id": item_id,
                "tool_call_id": tool_call_id,
                "name": name,
                "output": output,
                "success": success,
                "duration_ms": duration_ms,
            }),
        )
    }
}
