//! Bounded latest-attempt projection; older completions cannot replace newer attempts.

use super::{Observation, RunState, Ticket};
use chrono::Utc;
use parking_lot::RwLock;
use std::sync::{Arc, LazyLock};

static SHARED: LazyLock<Arc<ObservationStore>> = LazyLock::new(Arc::default);

#[derive(Default, Debug)]
pub(crate) struct ObservationStore(RwLock<Option<Observation>>);

pub(crate) fn shared_observations() -> Arc<ObservationStore> {
    Arc::clone(&SHARED)
}

impl ObservationStore {
    pub(crate) fn latest(&self) -> Option<Observation> {
        self.0.read().clone()
    }

    pub(crate) fn start(self: &Arc<Self>) -> Ticket {
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
