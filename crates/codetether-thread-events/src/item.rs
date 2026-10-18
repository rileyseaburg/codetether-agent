//! Assistant item lifecycle mapping.

use serde_json::json;

use codetether_thread_store::ThreadEvent;

use super::mapper::ThreadEventMapper;

impl ThreadEventMapper {
    pub(super) fn text_chunk(&mut self, text: &str) -> Vec<ThreadEvent> {
        let mut events = self.ensure_item_started("assistant_text");
        events.push(self.event(
            "item.delta",
            json!({
                "item_id": self.open_item_id(),
                "text": text,
            }),
        ));
        events
    }

    pub(super) fn text_completed(&mut self, text: &str) -> Vec<ThreadEvent> {
        let mut events = self.ensure_item_started("assistant_text");
        let item_id = self.open_item_id();
        events.push(self.event(
            "item.completed",
            json!({
                "item_id": item_id,
                "text": text,
            }),
        ));
        self.open_item_id = None;
        events
    }
}
