//! Feedback of local subtask outcomes into provider delegation.

use super::state::State;

pub(super) fn record(state: &State<'_, impl crate::Host>, id: &str, success: bool) {
    if let Some((provider, specialty)) = state.assignments.get(id) {
        state.executor.outcome(provider, specialty, success);
    }
}
