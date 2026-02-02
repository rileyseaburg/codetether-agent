//! Ralph - Autonomous PRD-driven agent loop
//!
//! Implementation of Geoffrey Huntley's ralph pattern:
//! `while :; do cat PROMPT.md | llm; done`
//!
//! Ralph iterates through PRD user stories, running quality gates after each,
//! and uses RLM to compress progress when context gets too large.

mod types;
mod ralph_loop;

pub use types::*;
pub use ralph_loop::*;
