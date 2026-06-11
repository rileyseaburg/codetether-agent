//! Stateful mapper from live session events to thread records.

use serde_json::Value;

use crate::session::thread_store::ThreadEvent;

use super::context::ThreadEventContext;
use super::ids::event_id;
use super::time::timestamp_ms;

pub(super) struct OpenTool {
    pub(super) id: String,
    pub(super) item_id: String,
}

/// Maps session, tool, and patch lifecycle events into thread records.
///
/// # Examples
///
/// ```
/// use codetether_agent::session::thread_events::{ThreadEventContext, ThreadEventMapper};
///
/// let context = ThreadEventContext::new("session-1", "turn-1");
/// let mut mapper = ThreadEventMapper::with_timestamp(context, 5);
/// let event = mapper.turn_started("hello");
/// assert_eq!(event.kind, "turn.started");
/// ```
pub struct ThreadEventMapper {
    pub(super) context: ThreadEventContext,
    pub(super) fixed_timestamp_ms: Option<u64>,
    pub(super) item_seq: u64,
    pub(super) open_item_id: Option<String>,
    pub(super) open_tools: Vec<OpenTool>,
    seq: u64,
}

impl ThreadEventMapper {
    /// Create a mapper using live wall-clock timestamps.
    pub fn new(context: ThreadEventContext) -> Self {
        Self::with_clock(context, None)
    }

    /// Create a mapper that stamps every event with `timestamp_ms`.
    pub fn with_timestamp(context: ThreadEventContext, timestamp_ms: u64) -> Self {
        Self::with_clock(context, Some(timestamp_ms))
    }

    fn with_clock(context: ThreadEventContext, fixed_timestamp_ms: Option<u64>) -> Self {
        Self {
            context,
            fixed_timestamp_ms,
            item_seq: 0,
            open_item_id: None,
            open_tools: Vec::new(),
            seq: 0,
        }
    }

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
