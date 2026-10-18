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
codetether_recall_query::impl_session_host!(
    storage_host::NativeHost,
    crate::session::Session,
    backfill::schedule,
    build::session
);
mod queue;
pub(crate) use codetether_session_recall::render;
pub(crate) mod search;
mod session_io;
pub(crate) mod storage_host;
codetether_recall_worker::impl_session_host!(
    storage_host::NativeHost,
    crate::session::Session,
    crate::session::listing::list_sessions_for_directory,
    crate::session::store::cursor::next,
    crate::session::store::projection::commit
);

pub(crate) use queue::{remove, schedule};

#[cfg(test)]
mod tests;
