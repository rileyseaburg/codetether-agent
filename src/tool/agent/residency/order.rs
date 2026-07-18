//! Shared residency ledger access and least-recently-used ordering.

use super::state::ResidencyState;
use std::sync::{Arc, LazyLock};

static STATE: LazyLock<Arc<ResidencyState>> = LazyLock::new(|| Arc::new(ResidencyState::default()));

pub(crate) fn touch(agent_id: &str) {
    STATE.touch(agent_id);
}

pub(crate) fn forget(agent_id: &str) {
    STATE.forget(agent_id);
}

pub(super) fn pop_candidate(protected: Option<&str>) -> Option<String> {
    STATE.pop_candidate(protected)
}

pub(super) fn resident_count() -> usize {
    STATE.resident_count()
}

pub(super) fn usage() -> usize {
    STATE.usage()
}

pub(super) fn shared() -> Arc<ResidencyState> {
    Arc::clone(&STATE)
}
