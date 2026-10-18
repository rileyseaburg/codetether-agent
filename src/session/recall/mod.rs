//! Fast, local retrieval over persisted session evidence.
//!
//! Session saves produce compact sidecars outside the transcript snapshot.
//! Queries load those sidecars through a workspace catalog and never invoke a
//! language model or deserialize full session histories.

mod backfill;
mod backfill_load;
mod backfill_policy;
mod backfill_worker;
mod build;
mod catalog_io;
mod direct_load;
pub(crate) mod hit;
mod indexed_session;
mod load;
mod paths;
pub(crate) mod prefetch;
mod queue;
mod queue_state;
mod queue_worker;
mod rank;
pub(crate) mod render;
pub(crate) mod search;
mod search_workspace;
mod session_io;
mod storage_host;
mod store;
mod store_lock;
mod tombstone;

pub(crate) use queue::{remove, schedule};

#[cfg(test)]
mod tests;
