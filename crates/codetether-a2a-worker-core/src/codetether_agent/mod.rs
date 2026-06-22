//! CodeTether agent runtime helpers used by operational diagnostics.
//!
//! # Examples
//!
//! ```rust
//! use codetether_a2a_worker_core::codetether_agent::subagent_orphans::SubAgentRunState;
//!
//! assert!(matches!(SubAgentRunState::Created, SubAgentRunState::Created));
//! ```

mod classifier;
mod record;
mod scanner;
mod state;
pub mod subagent_orphans;
