//! Command lifecycle mapping for shell tool calls.

use serde_json::{Value, json};

use crate::session::thread_store::ThreadEvent;

use super::mapper::ThreadEventMapper;

impl ThreadEventMapper {
    pub(super) fn command_started(&mut self, tool_call_id: &str, arguments: &str) -> ThreadEvent {
        let args = serde_json::from_str::<Value>(arguments).unwrap_or(Value::Null);
        let item_id = self.open_tool_item(tool_call_id);
        self.event(
            "command.started",
            json!({
                "item_id": item_id,
                "tool_call_id": tool_call_id,
                "command": args.get("command").and_then(Value::as_str),
                "cwd": args.get("cwd").and_then(Value::as_str),
            }),
        )
    }

    pub(super) fn command_completed(
        &mut self,
        tool_call_id: &str,
        success: bool,
        duration_ms: u64,
    ) -> ThreadEvent {
        let item_id = self.open_tool_item(tool_call_id);
        self.event(
            "command.completed",
            json!({
                "item_id": item_id,
                "tool_call_id": tool_call_id,
                "success": success,
                "duration_ms": duration_ms,
            }),
        )
    }
}
