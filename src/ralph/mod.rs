//! Ralph - Autonomous PRD-driven agent loop
//!
//! Implementation of Geoffrey Huntley's ralph pattern:
//! `while :; do cat PROMPT.md | llm; done`
//!
//! Ralph iterates through PRD user stories, running quality gates after each,
//! and uses RLM to compress progress when context gets too large.

mod delegation;
mod ralph_loop;
pub(crate) mod runtime_host;
use codetether_ralph_core as types;
pub use codetether_ralph_core::state_store;
pub use codetether_ralph_core::store_http;
pub use codetether_ralph_core::store_memory;
mod verification;

pub use delegation::*;
pub use ralph_loop::*;
pub use state_store::{RalphRunState, RalphRunSummary, RalphStateStore, StoryResultEntry};
pub use types::*;
