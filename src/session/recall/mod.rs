//! Fast, local retrieval over persisted session evidence.
//!
//! Session saves produce compact sidecars outside the transcript snapshot.
//! Queries load those sidecars through a workspace catalog and never invoke a
//! language model or deserialize full session histories.

mod backfill;
mod build;
use codetether_recall_index as indexed_session;
pub(crate) use codetether_session_recall as hit;
mod paths;
pub(crate) mod prefetch;
codetether_recall_context::impl_session_source!(
    crate::session::Session,
    crate::session::helper::runtime::prior_context_allowed_for_session
);
mod query_host;
mod queue;
pub(crate) use codetether_session_recall::render;
pub(crate) mod search;
mod session_io;
pub(crate) mod storage_host;
mod worker_host;

pub(crate) use queue::{remove, schedule};

#[cfg(test)]
mod tests;
