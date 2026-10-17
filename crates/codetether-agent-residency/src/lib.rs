//! Bounded live-agent loading, refresh, and safe LRU eviction.
//!
//! Implement [Storage] and [Host] for application persistence and session
//! overlays. [open] resolves or activates a child; [reserve] accounts for a
//! pending child and evicts only settled residents with no queued input.
//! The shared collaboration-state ledger, locks, and stale markers remain the
//! single owners of runtime state. [limit] and [rejected] supply spawn policy.
mod activate;
mod capacity;
mod ensure;
mod evict;
mod host;
mod storage;
mod thread_limit;
pub use capacity::reserve;
pub use codetether_agent_lifecycle::ResumeConfig;
pub use codetether_collaboration_tools::EnsureOpen;
pub use ensure::open;
pub use host::Host;
pub use storage::Storage;
pub use thread_limit::{limit, rejected};
