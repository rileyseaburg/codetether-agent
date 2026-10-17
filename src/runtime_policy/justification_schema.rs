//! Compatibility exports for shared schema justification policy.

pub use codetether_runtime_policy::justification_schema::{apply, note_access_mode};

#[cfg(test)]
#[path = "justification_schema_tests.rs"]
mod tests;
