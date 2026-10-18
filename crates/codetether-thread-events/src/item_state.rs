//! Assistant item allocation and current item lookup.
use super::{ids::item_id, mapper::ThreadEventMapper};
use codetether_thread_store::ThreadEvent;
use serde_json::json;
impl ThreadEventMapper {
    pub(super) fn ensure_item_started(&mut self, item_type: &str) -> Vec<ThreadEvent> {
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

    pub(super) fn open_item_id(&self) -> String {
        self.open_item_id.clone().unwrap_or_default()
    }
}
