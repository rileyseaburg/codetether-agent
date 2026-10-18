//! Stamp the next record with stable context IDs and sequence.
use super::{ids::event_id, mapper::ThreadEventMapper, time::timestamp_ms};
use codetether_thread_store::ThreadEvent;
use serde_json::Value;
impl ThreadEventMapper {
    pub(super) fn event(&mut self, kind: &str, payload: Value) -> ThreadEvent {
        self.seq += 1;
        ThreadEvent {
            event_id: event_id(&self.context.turn_id, self.seq),
            thread_id: self.context.thread_id.clone(),
            session_id: self.context.session_id.clone(),
            turn_id: self.context.turn_id.clone(),
            kind: kind.to_string(),
            timestamp_ms: self.fixed_timestamp_ms.unwrap_or_else(timestamp_ms),
            payload,
        }
    }
}
