//! Shared bus contracts, live delivery, registry, history and durable replay.
//!
//! Application startup and external sinks remain adapters.
use chrono::{DateTime, Utc};
use codetether_a2a_core::{Artifact, Part, TaskState};
use serde::{Deserialize, Serialize};
/// Process-wide bus installation and lookup.
pub mod global;
mod message;
pub use message::BusMessage;
mod envelope;
pub use envelope::BusEnvelope;
pub mod durability_class;
pub mod payload;
pub mod speech;
pub mod tool_catalog;

/// Bounded message history with cursor draining.
pub mod recorder;
/// Agent cards indexed by live bus identity.
pub mod registry;

/// Partitioned replay and background append contracts.
pub mod durable_log;
/// JSONL-backed durable coordination storage.
pub mod durable_log_file;

mod runtime;
pub use runtime::{AgentBus, BusHandle};

/// Correlated multi-agent handoffs and lifecycle registration.
pub mod relay;

/// Training-record projection and batch serialization independent of storage.
pub mod training;

/// Correlated peer-message publication.
pub mod peer_messages;

/// Task-correlated messages from A2A server endpoints.
pub mod a2a_messages;

/// Correlated tool request/response event publication.
pub mod tool_events;

/// Correlated assistant reasoning and text events.
pub mod assistant_events;
