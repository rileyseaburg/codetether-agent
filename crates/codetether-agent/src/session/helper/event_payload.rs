//! Shared bounded tool payloads for live session events.
#[cfg(not(test))]
pub(crate) use codetether_stream_bounds::bounded_tool_arguments;

#[cfg(test)]
include!("event_payload_test_imports.rs");
