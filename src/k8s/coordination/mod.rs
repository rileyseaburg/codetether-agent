//! Distributed ownership enforcement for Kubernetes mutations.
//!
//! Every mutating operation acquires a namespace-scoped Kubernetes `Lease`,
//! renews it while running, and releases it with ownership preconditions.

mod acquire;
#[cfg(test)]
mod api_conflict_test;
#[cfg(test)]
mod api_create_test;
#[cfg(test)]
mod api_release_test;
#[cfg(test)]
mod api_takeover_test;
mod claim;
mod config;
mod coordinator;
mod decision;
#[cfg(test)]
mod decision_tests;
mod error;
mod guard;
mod guard_task;
mod identity;
mod manifest;
#[cfg(test)]
mod mock_api;
mod renew;
mod repository;
mod resource;
#[cfg(test)]
mod test_support;

pub use claim::MutationClaim;
pub use coordinator::MutationCoordinator;
pub use error::LeaseConflict;
