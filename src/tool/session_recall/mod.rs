//! `session_recall` tool: RLM-powered recall over persisted session history.
//!
//! Split into focused submodules to respect the 50-line file limit.

mod context;
mod faults;
mod rlm_run;
mod schema;

mod tool_struct;
pub use tool_struct::SessionRecallTool;
