//! Bounded latest-attempt projection; older completions cannot replace newer attempts.

use super::{Observation, RunState, Ticket};
use chrono::Utc;
use parking_lot::RwLock;
use std::sync::{Arc, LazyLock};

static SHARED: LazyLock<Arc<ObservationStore>> = LazyLock::new(Arc::default);

/// Latest-started verification receipt; use start on an Arc to obtain a ticket.
/// # Examples
/// ```
/// let store = std::sync::Arc::new(codetether_verifier_state::observation::ObservationStore::default());
/// assert!(store.latest().is_none());
/// let ticket = store.start();
/// assert_eq!(store.latest().unwrap().verification_id, ticket.id());
/// ```
#[derive(Default, Debug)]
pub struct ObservationStore(RwLock<Option<Observation>>);

/// Return the process-wide observation store used by the harness.
/// # Returns
/// A cloned Arc referencing the same store.
/// # Examples
/// ```
/// let store = codetether_verifier_state::observation::shared_observations();
/// let other = codetether_verifier_state::observation::shared_observations();
/// assert!(std::sync::Arc::ptr_eq(&store, &other));
/// ```
pub fn shared_observations() -> Arc<ObservationStore> {
    Arc::clone(&SHARED)
}

impl ObservationStore {
    /// Snapshot the newest started attempt, if any.
    pub fn latest(&self) -> Option<Observation> {
        self.0.read().clone()
    }

    /// Start a new latest attempt and return its identity-preserving ticket.
    pub fn start(self: &Arc<Self>) -> Ticket {
        let mut latest = self.0.write();
        let record = Observation {
            verification_id: format!("goal-verifier-{}", uuid::Uuid::new_v4()),
            identity_source: "harness_provider_resolution",
            requested_model: None,
            resolved_provider: None,
            resolved_model: None,
            state: RunState::Resolving,
            started_at: Utc::now(),
            finished_at: None,
        };
        *latest = Some(record.clone());
        Ticket {
            store: Arc::clone(self),
            record,
        }
    }

    pub(super) fn publish(&self, record: &Observation) {
        let mut latest = self.0.write();
        if latest
            .as_ref()
            .is_some_and(|item| item.verification_id == record.verification_id)
        {
            *latest = Some(record.clone());
        }
    }
}
