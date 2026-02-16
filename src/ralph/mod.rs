//! Ralph - Autonomous PRD-driven agent loop
//!
//! Implementation of Geoffrey Huntley's ralph pattern:
//! `while :; do cat PROMPT.md | llm; done`
//!
//! Ralph iterates through PRD user stories, running quality gates after each,
//! and uses RLM to compress progress when context gets too large.

mod ralph_loop;
pub mod state_store;
pub mod store_http;
pub mod store_memory;
mod types;

pub use ralph_loop::*;
pub use state_store::{RalphRunState, RalphRunSummary, RalphStateStore, StoryResultEntry};
pub use types::*;
