//! Tool metadata mapping.

use serde_json::json;

use crate::session::thread_store::ThreadEvent;

use super::mapper::ThreadEventMapper;

impl ThreadEventMapper {
    pub(super) fn tool_metadata_events(
        &mut self,
        tool_call_id: &str,
        name: &str,
        metadata: serde_json::Value,
    ) -> Vec<ThreadEvent> {
        let mut events = vec![self.tool_metadata(tool_call_id, name, metadata.clone())];
        if name == "apply_patch" {
            events.extend(self.patch_metadata_events(tool_call_id, &metadata));
        }
        events
    }

    pub(super) fn tool_metadata(
        &mut self,
        tool_call_id: &str,
        name: &str,
        metadata: serde_json::Value,
    ) -> ThreadEvent {
        let item_id = self.open_tool_item(tool_call_id);
        self.event(
            "tool.metadata",
            json!({
                "item_id": item_id,
                "tool_call_id": tool_call_id,
                "name": name,
                "metadata": metadata,
            }),
        )
    }
}
