//! Native delegation locking and outcome recording.
use super::SwarmExecutor;
use crate::provider::ProviderRegistry;
pub(super) fn choose(
    executor: &SwarmExecutor,
    providers: &ProviderRegistry,
    specialty: &str,
    provider: &str,
    model: &str,
) -> (String, String) {
    executor
        .delegation
        .as_ref()
        .map(|delegation| {
            let guard = delegation.lock().unwrap_or_else(|error| error.into_inner());
            crate::swarm::delegation::choose_provider_for_subtask(
                providers, &guard, specialty, provider, model,
            )
        })
        .unwrap_or_else(|| (provider.to_string(), model.to_string()))
}
pub(super) fn outcome(executor: &SwarmExecutor, provider: &str, specialty: &str, success: bool) {
    if let Some(delegation) = executor.delegation.as_ref() {
        crate::swarm::delegation_outcome::record_subtask_outcome(
            delegation, provider, specialty, success,
        );
    }
}
