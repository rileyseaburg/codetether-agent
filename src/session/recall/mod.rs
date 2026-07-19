//! Fast, local retrieval over persisted session evidence.
//!
//! Session saves produce compact sidecars outside the transcript snapshot.
//! Queries load those sidecars through a workspace catalog and never invoke a
//! language model or deserialize full session histories.

mod atomic;
mod backfill;
mod backfill_load;
mod backfill_policy;
mod backfill_worker;
mod build;
mod catalog;
mod catalog_io;
mod direct_load;
mod document;
mod excerpt;
pub(crate) mod hit;
mod indexed_session;
mod load;
mod paths;
pub(crate) mod prefetch;
mod queue;
mod queue_state;
mod queue_worker;
mod rank;
mod rank_score;
pub(crate) mod render;
pub(crate) mod search;
mod search_workspace;
mod session_io;
mod store;
mod store_freshness;
mod store_lock;
mod tokens;
mod tombstone;

pub(crate) use queue::{remove, schedule};

#[cfg(test)]
mod tests;
