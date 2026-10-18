//! Coalesced recall indexing and opt-in bounded workspace backfill.
//!
//! [schedule] coalesces pending session IDs; [remove] marks deletion before
//! removing queued work and sidecars. [schedule_backfill] runs at most once per
//! workspace key. [Host] binds session listing, cursor delivery and commits.
mod backfill;
mod backfill_load;
mod backfill_policy;
mod backfill_worker;
mod host;
mod job;
mod queue;
mod queue_state;
mod settle;
mod state;
mod worker;
pub use backfill::schedule as schedule_backfill;
pub use host::Host;
pub use queue::{remove, schedule};

/// Implementation support for application-local session bindings.
#[doc(hidden)]
pub mod session_binding;
