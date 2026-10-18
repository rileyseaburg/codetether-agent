//! Shared tool lifecycle event emitters.
#[cfg(not(test))]
pub(in crate::session::helper) use codetether_tool_reporting::start;

#[cfg(test)]
include!("tool_event_emit_test_imports.rs");
