//! Native recall services with application session access.
//!
//! The native recall crate owns storage, projection, querying and workers.
//! This module binds the existing session and prior-context policy.
#[cfg(test)]
mod queue;
#[cfg(test)]
pub(crate) mod search;
mod session;
#[cfg(test)]
use codetether_native_recall::build;
pub(crate) use codetether_native_recall::prefetch;
pub(crate) mod storage_host;
codetether_recall_context::impl_session_source!(
    crate::session::Session,
    crate::session::helper::runtime::prior_context_allowed_for_session
);
#[cfg(test)]
mod tests;

#[cfg(test)]
include!("mod_test_imports.rs");
