//! Orphan-aware status helpers for spawned CodeTether sub-agents.
//!
//! This facade keeps the public API stable while the state, record,
//! classifier, and scanner stay in small cohesive modules.
//!
//! # Examples
//!
//! ```rust
//! use codetether_a2a_worker_core::codetether_agent::subagent_orphans::SubAgentRunState;
//!
//! assert_eq!(SubAgentRunState::Orphaned.as_str(), "Orphaned");
//! ```

pub use super::classifier::classify_child_session_value;
pub use super::record::SubAgentRunRecord;
pub use super::scanner::{classify_child_session_file, scan_orphaned_child_sessions};
pub use super::state::SubAgentRunState;
