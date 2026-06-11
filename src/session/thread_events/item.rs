//! Assistant item lifecycle mapping.

use serde_json::json;

use crate::session::thread_store::ThreadEvent;

use super::ids::item_id;
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

    fn ensure_item_started(&mut self, item_type: &str) -> Vec<ThreadEvent> {
        if self.open_item_id.is_some() {
            return Vec::new();
        }
        self.item_seq += 1;
        let id = item_id(&self.context.turn_id, self.item_seq);
        self.open_item_id = Some(id.clone());
        vec![self.event(
            "item.started",
            json!({
                "item_id": id,
                "item_type": item_type,
            }),
        )]
    }

    fn open_item_id(&self) -> String {
        self.open_item_id.clone().unwrap_or_default()
    }
}
