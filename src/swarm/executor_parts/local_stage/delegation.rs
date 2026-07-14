//! Provider selection using optional delegation state.

use super::state::State;
use crate::provider::Provider;
use crate::swarm::SubTask;
use std::sync::Arc;

pub(super) fn select(state: &State<'_>, task: &SubTask) -> (String, String, Arc<dyn Provider>) {
    let specialty = task.specialty.as_deref().unwrap_or("General");
    let selected = state
        .executor
        .delegation
        .as_ref()
        .map(|delegation| {
            let guard = delegation.lock().unwrap_or_else(|error| error.into_inner());
            crate::swarm::delegation::choose_provider_for_subtask(
                state.providers,
                &guard,
                specialty,
                &state.provider_name,
                &state.model,
            )
        })
        .unwrap_or_else(|| (state.provider_name.clone(), state.model.clone()));
    let provider = state
        .providers
        .get(&selected.0)
        .unwrap_or_else(|| Arc::clone(&state.fallback_provider));
    (selected.0, selected.1, provider)
}
