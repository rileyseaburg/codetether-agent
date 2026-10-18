//! Bounded streaming accumulators and UTF-8-safe live previews.
//!
//! [ensure_text_room] and [ensure_tool_args_room] reject runaway accumulators.
//! [snapshot_for_send] bounds assistant previews; [bounded_tool_output] and
//! [bounded_tool_arguments] bound event payloads while full history stays intact.
mod accumulator;
mod event;
mod limits;
mod snapshot;
pub use accumulator::{ensure_text_room, ensure_tool_args_room};
pub use event::{bounded_tool_arguments, bounded_tool_output};
pub use limits::*;
pub use snapshot::snapshot_for_send;
#[cfg(test)]
mod tests;
