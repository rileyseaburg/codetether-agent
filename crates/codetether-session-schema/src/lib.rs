//! Shared session schema, serialization and data lifecycle operations.
//!
//! [declare_session!] defines the schema while keeping the generated type local
//! to the application that owns its inherent operations. [DEFAULT_MAX_STEPS]
//! supplies the established fallback for bounded delegated runs.
//! [impl_session_lifecycle!] adds construction, provenance and transcript methods.
mod checkpoint;
mod declare;
mod header;
mod lifecycle;
mod models;
mod persistence;
mod storage;
mod title;
pub use header::{HeaderSource, header};

/// Step budget for bounded delegated runs that do not specify one.
/// Primary sessions may still run without a step limit.
pub const DEFAULT_MAX_STEPS: usize = 50;

/// Stable macro dependencies; not intended for direct application use.
#[doc(hidden)]
pub mod __private {
    pub use anyhow;
    pub use async_trait::async_trait;
    pub use chrono::{DateTime, Utc};
    pub use codetether_agent_core::ToolUse;
    pub use codetether_bus_core::AgentBus;
    pub use codetether_bus_core::global;
    pub use codetether_config as config;
    pub use codetether_context_policy::{pages::PageKind, summary_index::SummaryIndex};
    pub use codetether_provenance::ClaimProvenance;
    pub use codetether_provider_core::{Message, Provider, Usage};
    pub use codetether_provider_policy as provider_policy;
    pub use codetether_run_checkpoint as checkpoint;
    pub use codetether_session_codec as codec;
    pub use codetether_session_history::History;
    pub use codetether_session_location as location;
    pub use codetether_session_metadata as metadata;
    pub use codetether_session_metadata::SessionMetadata;
    pub use codetether_session_model_policy as model_policy;
    pub use codetether_session_store as store;
    pub use codetether_session_store::State;
    pub use codetether_session_store::window;
    pub use codetether_session_title as titles;
    pub use codetether_tracked_vec::{TrackedVec, tail};
    pub use serde;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json;
}
