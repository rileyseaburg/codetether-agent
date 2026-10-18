//! Evict the oldest open call before admitting a new one at capacity.
use super::observe::MAX_OPEN_CALLS;
use super::{OpenToolCall, ToolCallTracker};
impl ToolCallTracker {
    pub(super) fn insert(&mut self, request_id: &str, call: OpenToolCall) {
        if self.open.len() >= MAX_OPEN_CALLS
            && !self.open.contains_key(request_id)
            && let Some(oldest) = self
                .open
                .iter()
                .min_by_key(|(_, call)| call.started_at)
                .map(|(id, _)| id.clone())
        {
            self.open.remove(&oldest);
        }
        self.open.insert(request_id.to_string(), call);
    }
}
