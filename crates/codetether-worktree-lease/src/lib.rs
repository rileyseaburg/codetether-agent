//! Atomic path-scoped worktree leases for a single authoritative coordinator.
//!
//! [LeaseRegistry] acquires multiple paths atomically, rejects workspace-wide
//! claims, and supports bounded notification-driven waits. Serializable
//! [CoordinationRequest], [CoordinationReply] and [WorktreeLease] preserve the
//! client/server contract; transport and authentication belong to the caller.
mod acquire;
mod claim;
mod conflict;
mod key;
mod lifecycle;
mod record;
mod registry;
mod reply;
mod reply_wait;
mod request;
mod signal;
mod time;
mod wait;
pub use codetether_mutation_scope::workspace_claim;
pub use record::WorktreeLease;
pub use registry::LeaseRegistry;
pub use reply::CoordinationReply;
pub use request::CoordinationRequest;
/// Maximum wait for a conflicting lease, in milliseconds.
pub const ACQUIRE_WAIT_MILLIS: u64 = 60_000;
#[cfg(test)]
mod tests;
