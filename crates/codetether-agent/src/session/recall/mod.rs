//! Native recall services with application session access.
//!
//! The native recall crate owns storage, projection, querying and workers.
//! This module binds the existing session and prior-context policy.
mod session;
#[cfg(test)]
use codetether_native_recall::build;
pub(crate) use codetether_native_recall::prefetch;
pub(crate) use codetether_session_recall as hit;
pub(crate) use codetether_session_recall::render;
mod queue;
pub(crate) mod search;
pub(crate) mod storage_host;
pub(crate) use queue::{remove, schedule};
codetether_recall_context::impl_session_source!(
    crate::session::Session,
    crate::session::helper::runtime::prior_context_allowed_for_session
);
#[cfg(test)]
mod tests;
