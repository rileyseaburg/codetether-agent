//! Shared session field layout and serialization contract.
//!
//! [declare_session!] defines the schema while keeping the generated type local
//! to the application that owns its inherent operations. [DEFAULT_MAX_STEPS]
//! supplies the established fallback for bounded delegated runs.
mod declare;

/// Step budget for bounded delegated runs that do not specify one.
/// Primary sessions may still run without a step limit.
pub const DEFAULT_MAX_STEPS: usize = 50;

/// Stable macro dependencies; not intended for direct application use.
#[doc(hidden)]
pub mod __private {
    pub use chrono::{DateTime, Utc};
    pub use codetether_agent_core::ToolUse;
    pub use codetether_bus_core::AgentBus;
    pub use codetether_context_policy::{pages::PageKind, summary_index::SummaryIndex};
    pub use codetether_provider_core::{Message, Usage};
    pub use codetether_session_metadata::SessionMetadata;
    pub use codetether_session_store::State;
    pub use codetether_tracked_vec::{TrackedVec, tail};
    pub use serde;
    pub use serde::{Deserialize, Serialize};
}
