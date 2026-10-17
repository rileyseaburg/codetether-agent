//! Durable FIFO mailbox state and dispatch for child agents.
//!
//! A [Host] supplies snapshot persistence and execution. [enqueue] durably
//! accepts input; [dispatch_next] claims it, and [finished] acknowledges or
//! releases it after a turn. All callers share one in-memory queue registry.
mod claim;
mod completion;
mod dispatcher;
mod host;
mod input;
mod item;
mod receipt;
mod snapshot;
mod store;
mod store_lifecycle;
pub use claim::next as claim;
pub use completion::finished;
pub use dispatcher::next as dispatch_next;
pub use host::Host;
pub use input::EnqueueInput;
pub use item::{DeliveryState, Item};
pub use receipt::{ack, release};
pub use snapshot::Snapshot;
pub use store::enqueue;
pub use store_lifecycle::{clear, hydrate, pending, reparent_owner, reset_cache};
mod submission;
pub use submission::Submission;
