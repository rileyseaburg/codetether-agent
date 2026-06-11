//! Patch metadata expansion.

use crate::session::thread_store::ThreadEvent;

use super::mapper::ThreadEventMapper;

impl ThreadEventMapper {
    pub(super) fn patch_metadata_events(
        &mut self,
        patch_id: &str,
        metadata: &serde_json::Value,
    ) -> Vec<ThreadEvent> {
        let mut events = vec![self.patch_completed(patch_id, metadata)];
        if let Some(id) = metadata
            .get("approval_request_id")
            .and_then(serde_json::Value::as_str)
        {
            events.push(self.patch_approval_required(id, patch_id));
        }
        events
    }
}
