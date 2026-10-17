//! Compatibility exports for shared read-only tool classification.

pub use codetether_tool_core::readonly::is_read_only;

#[cfg(test)]
#[path = "readonly_tests.rs"]
mod tests;
