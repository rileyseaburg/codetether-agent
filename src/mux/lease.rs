//! Authoritative shared-worktree leases owned by one mux server.

#[path = "lease/acquire.rs"]
mod acquire;
#[path = "lease/claim.rs"]
mod claim;
#[path = "lease/key.rs"]
mod key;
#[path = "lease/lifecycle.rs"]
mod lifecycle;
#[path = "lease/record.rs"]
mod record;
#[path = "lease/registry.rs"]
mod registry;
#[path = "lease/reply.rs"]
mod reply;
#[path = "lease/request.rs"]
mod request;
#[path = "lease/time.rs"]
mod time;

pub(crate) use record::WorktreeLease;
pub(in crate::mux) use registry::LeaseRegistry;
pub(crate) use reply::CoordinationReply;
pub(in crate::mux) use request::CoordinationRequest;

#[cfg(test)]
mod tests;
