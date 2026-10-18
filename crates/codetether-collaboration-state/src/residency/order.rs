//! Shared residency ledger access and least-recently-used ordering.

use super::state::ResidencyState;
use std::sync::{Arc, LazyLock};

static STATE: LazyLock<Arc<ResidencyState>> = LazyLock::new(|| Arc::new(ResidencyState::default()));

/// Move an agent to the newest end of the resident order.
/// # Arguments
/// * `agent_id` — Agent identity for this operation.
/// # Returns
/// Unit after inserting or moving the identity.
/// # Examples
/// See [super::Slot] for reserving and committing residency.
pub fn touch(agent_id: &str) {
    STATE.touch(agent_id);
}

/// Remove an agent from the shared residency ledger.
/// # Arguments
/// * `agent_id` — Agent identity to remove.
/// # Returns
/// Unit; removing an absent identity has no effect.
/// # Examples
/// ```
/// codetether_collaboration_state::residency::forget("docs-forgotten");
/// ```
pub fn forget(agent_id: &str) {
    STATE.forget(agent_id);
}

/// Remove the oldest resident that is not protected.
/// # Arguments
/// * `protected` — Agent identity for this operation.
/// # Returns
/// The selected identity, or none when no candidate is available.
/// # Examples
/// See [super::Slot] for reserving and committing residency.
pub fn pop_candidate(protected: Option<&str>) -> Option<String> {
    STATE.pop_candidate(protected)
}

/// Count committed resident identities.
/// # Arguments
/// This function takes no arguments.
/// # Returns
/// The number of committed residents.
/// # Examples
/// See [super::Slot] for reserving and committing residency.
pub fn resident_count() -> usize {
    STATE.resident_count()
}

/// Count committed residents and pending reservations.
/// # Arguments
/// This function takes no arguments.
/// # Returns
/// The combined residency usage.
/// # Examples
/// See [super::Slot] for reserving and committing residency.
pub fn usage() -> usize {
    STATE.usage()
}

pub(super) fn shared() -> Arc<ResidencyState> {
    Arc::clone(&STATE)
}
