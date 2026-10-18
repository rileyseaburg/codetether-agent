//! Stateful mapper from live session events to thread records.

use super::context::ThreadEventContext;

pub(super) struct OpenTool {
    pub(super) id: String,
    pub(super) item_id: String,
}

/// Maps session, tool, and patch lifecycle events into thread records.
///
/// # Examples
///
/// ```
/// use codetether_thread_events::{ThreadEventContext, ThreadEventMapper};
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
    pub(super) seq: u64,
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
}
