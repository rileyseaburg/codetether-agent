//! Worker event sink sink.
use super::*;

/// Returns the installed sink, if any.
pub(super) fn sink() -> Option<EventSink> {
    ACTIVE_SINK.read().ok().and_then(|slot| slot.clone())
}
