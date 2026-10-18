//! Provider selection using optional delegation state.

use super::state::State;
use codetether_provider_core::Provider;
use codetether_swarm_core::SubTask;
use std::sync::Arc;

pub(super) fn select<H: crate::Host>(
    state: &State<'_, H>,
    task: &SubTask,
) -> (String, String, Arc<dyn Provider>) {
    let specialty = task.specialty.as_deref().unwrap_or("General");
    let selected = state.executor.choose(
        state.providers,
        specialty,
        &state.provider_name,
        &state.model,
    );
    let provider = H::get(state.providers, &selected.0)
        .unwrap_or_else(|| Arc::clone(&state.fallback_provider));
    (selected.0, selected.1, provider)
}
