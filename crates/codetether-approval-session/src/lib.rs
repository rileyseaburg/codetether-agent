//! Live approval handoff through session event channels.
//!
//! [request] publishes an approval event and awaits its decision. [decide]
//! resolves a pending request, and [latest_id] locates the newest pending one.
//! Pending state and cancellation cleanup are owned by approval-core.
mod request;
pub use codetether_approval_core::interaction::pending::decide;
/// Query used by frontends to discard requests that have already resolved.
#[doc(hidden)]
pub use codetether_approval_core::interaction::pending_query::is_pending;
pub use codetether_approval_core::interaction::pending_query::latest_id;
pub use codetether_approval_core::interaction::{LiveApprovalDecision, LiveApprovalRequest};
pub use request::request;

#[cfg(test)]
mod order_tests;
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;
