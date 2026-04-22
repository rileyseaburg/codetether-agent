//! Agent-facing tool for managing the session's goal and task log.
//!
//! Writes append-only events to
//! `<sessions_dir>/<CODETETHER_SESSION_ID>.tasks.jsonl` via
//! [`crate::session::tasks::TaskLog`]. The session id is supplied via
//! the `CODETETHER_SESSION_ID` environment variable which the agent
//! runtime already sets for every turn (see `bash.rs`).

mod handlers;
mod params;
mod tool;

#[allow(unused_imports)]
pub use tool::SessionTaskTool;
