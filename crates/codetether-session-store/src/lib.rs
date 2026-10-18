//! Transactional kernel for indexed session persistence.
//!
//! [connection] opens and upgrades the shared SQLite database. [transaction]
//! commits [Batch] envelopes with optimistic revisions and retry identities.
//! [state] holds per-session checkpoints; native encoders and hydration adapters
//! provide [Snapshot] data and header encoding; this crate owns delta selection,
//! schema, writes, checkpoint acknowledgement and indexed reads.
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

mod acknowledge;
pub mod cursor_read;
pub mod delete;
mod delivery;
pub mod header_read;
pub mod projection;
pub mod summaries;
mod ticket;
pub use acknowledge::acknowledge;
pub use delivery::Delivery;
pub use ticket::Ticket;

mod checkpoint;
mod encode;
mod prepare;
mod save;
mod snapshot;
pub use checkpoint::clean;
pub use prepare::prepare;
pub use save::save;
pub use snapshot::Snapshot;

mod restoration;
pub use restoration::{Restore, TailLoad, load, load_sync, restore_consumer};

pub mod legacy;
