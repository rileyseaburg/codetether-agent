//! Transactional kernel for indexed session persistence.
//!
//! [connection] opens and upgrades the shared SQLite database. [transaction]
//! commits [Batch] envelopes with optimistic revisions and retry identities.
//! [state] holds per-session checkpoints; native encoders and hydration adapters
//! construct records while this crate owns schema, writes and indexed reads.
pub mod batch;
pub mod blobs;
pub mod connection;
pub mod constraints;
pub mod events;
pub mod id;
pub mod identity;
pub mod marker;
pub mod read;
pub mod records;
pub mod schema;
pub mod state;
pub mod tool_state;
pub mod transaction;
mod upgrade;
pub use batch::{Batch, Records};
pub use state::{Checkpoint, State};
