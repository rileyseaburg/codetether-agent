//! Compaction lifecycle emission entry points.
pub(super) use super::event_completed::emit_completed;
pub(super) use super::event_send::emit;
pub(super) use super::event_started::emit_started;
