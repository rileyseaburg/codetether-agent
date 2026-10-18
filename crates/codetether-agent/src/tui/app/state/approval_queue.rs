//! Native integration for the shared pending approval display queue.
pub(crate) use codetether_approval_edit as edit_session;
#[path = "approval_queue/reconcile.rs"]
mod reconcile;
#[cfg(test)]
pub(crate) use codetether_approval_queue::reset;
#[cfg(not(test))]
pub(crate) use codetether_approval_queue::{active, active_id, len};
pub(crate) use reconcile::remove_stale;

#[cfg(test)]
include!("approval_queue_test_imports.rs");
