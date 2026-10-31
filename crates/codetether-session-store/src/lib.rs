//! Transactional kernel for indexed session persistence.
//!
//! [connection] opens and upgrades the shared SQLite database. [transaction]
//! commits [Batch] envelopes with optimistic revisions and retry identities.
//! [state] holds per-session checkpoints; native encoders and hydration adapters
//! provide [Snapshot] data and header encoding; this crate owns delta selection,
//! schema, writes, checkpoint acknowledgement and indexed reads.
mod api;
pub use api::*;
pub mod batch;
pub mod blobs;
pub mod connection;
pub mod constraints;
/// Bounded, per-database serialization of session persistence writes.
pub mod coordinator;
pub mod events;
pub mod id;
pub mod identity;
pub mod marker;
pub mod pin_edits;
mod pin_read;
pub mod read;
pub mod records;
pub mod schema;
pub mod state;
pub mod tool_state;
pub mod transaction;
mod upgrade;

mod acknowledge;
mod cursor_prepare;
pub mod cursor_read;
pub mod delete;
mod delivery;
pub mod header_read;
pub mod projection;
pub mod summaries;
mod ticket;

mod checkpoint;
mod encode;
mod prepare;
mod save;
mod save_checkpoint;
mod snapshot;

mod restoration;

pub mod legacy;

pub mod metadata;
/// Read-only canonical user-message evidence for goal provenance.
pub mod user_source;
pub mod window;

/// Legacy-aware consumer snapshots from explicit or resolved session locations.
pub mod consumer;
mod fold_messages;
/// Persistence lifecycle and native indexing notifications.
pub mod lifecycle;
mod messages;
