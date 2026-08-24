//! Decode persisted approval decisions carried by durable tool metadata.

use crate::session::thread_store::ThreadEvent;

use super::mapper::ThreadEventMapper;

impl ThreadEventMapper {
    pub(super) fn approval_metadata(
        &mut self,
        metadata: &serde_json::Value,
    ) -> Option<ThreadEvent> {
        let value = metadata.get("approval_decision")?.clone();
        let decision = serde_json::from_value(value).ok()?;
        Some(self.approval_decided(&decision))
    }
}
