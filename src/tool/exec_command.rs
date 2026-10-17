//! Compatibility export and application approval regression tests.
pub use codetether_exec_command::ExecCommandTool;
#[cfg(test)]
#[path = "exec_command/policy.rs"]
mod policy;
