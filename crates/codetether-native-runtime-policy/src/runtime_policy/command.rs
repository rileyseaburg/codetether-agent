//! Compatibility exports for shared runtime permission policy.

pub use codetether_runtime_policy::command::{is_read_only_command, is_read_only_shell, value};

#[cfg(test)]
#[path = "command_exec_tests.rs"]
mod exec_tests;
