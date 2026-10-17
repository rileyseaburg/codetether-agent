//! Compatibility exports for shared non-bypassable command checks.
pub(crate) use codetether_command_guard::{result, result_for_args};
#[cfg(test)]
#[path = "shell_command_guard_tests.rs"]
mod tests;
