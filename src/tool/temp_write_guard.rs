//! Shared temporary-directory write guard and application boundary tests.
pub(crate) use codetether_tool_core::temp_write_guard::denied_roots;
pub use codetether_tool_core::temp_write_guard::{denied_reason, denied_result};
#[cfg(test)]
#[path = "temp_write_guard_boundary_tests.rs"]
mod boundary_tests;
