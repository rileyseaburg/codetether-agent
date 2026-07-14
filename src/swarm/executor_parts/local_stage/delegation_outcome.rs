//! Feedback of local subtask outcomes into provider delegation.

use super::state::State;

pub(super) fn record(state: &State<'_>, id: &str, success: bool) {
    if let (Some(delegation), Some((provider, specialty))) = (
        state.executor.delegation.as_ref(),
        state.assignments.get(id),
    ) {
        crate::swarm::delegation_outcome::record_subtask_outcome(
            delegation, provider, specialty, success,
        );
    }
}
