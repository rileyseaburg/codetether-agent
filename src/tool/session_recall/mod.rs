//! `session_recall` tool: immediate local evidence with optional RLM synthesis.
//!
//! Split into focused submodules to respect the 50-line file limit.

mod answer;
mod args;
mod description;
mod execute;
mod faults;
mod rlm_run;
mod schema;

mod tool_impl;
mod tool_struct;
pub use tool_struct::SessionRecallTool;

#[cfg(test)]
mod tests;
