//! Thread-event mapping for a sampling reconnect.

use serde_json::json;

use crate::session::thread_store::ThreadEvent;

use super::mapper::ThreadEventMapper;

impl ThreadEventMapper {
    pub(super) fn stream_retry(
        &mut self,
        attempt: u32,
        max_restarts: u32,
        reason: &str,
    ) -> Vec<ThreadEvent> {
        let abandoned_item_id = self.open_item_id.take();
        vec![self.event(
            "stream.retry",
            json!({
                "attempt": attempt,
                "max_restarts": max_restarts,
                "reason": reason,
                "abandoned_item_id": abandoned_item_id,
            }),
        )]
    }
}
