//! Fast, local retrieval over persisted session evidence.
//!
//! Session saves produce compact sidecars outside the transcript snapshot.
//! Queries load those sidecars through a workspace catalog and never invoke a
//! language model or deserialize full session histories.

mod backfill;
mod build;
pub(crate) mod hit;
use codetether_recall_index as indexed_session;
mod paths;
pub(crate) mod prefetch;
mod prefetch_host;
mod query_host;
mod queue;
pub(crate) mod render;
pub(crate) mod search;
mod session_io;
mod storage_host;
mod worker_host;

pub(crate) use queue::{remove, schedule};

#[cfg(test)]
mod tests;
