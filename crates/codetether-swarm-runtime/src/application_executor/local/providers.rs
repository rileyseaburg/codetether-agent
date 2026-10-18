//! Native delegation locking and outcome recording.
use super::LocalStageHost;
use crate::SwarmExecutor;
pub(super) fn choose<H: LocalStageHost>(
    executor: &SwarmExecutor<H>,
    providers: &H::Providers,
    specialty: &str,
    provider: &str,
    model: &str,
) -> (String, String) {
    executor
        .inner
        .delegation
        .as_ref()
        .map(|delegation| {
            let guard = delegation.lock().unwrap_or_else(|error| error.into_inner());
            codetether_swarm_delegation::choose_provider_for_subtask(
                || H::list(providers),
                &guard,
                specialty,
                provider,
                model,
            )
        })
        .unwrap_or_else(|| (provider.to_string(), model.to_string()))
}
pub(super) fn outcome<H: LocalStageHost>(
    executor: &SwarmExecutor<H>,
    provider: &str,
    specialty: &str,
    success: bool,
) {
    if let Some(delegation) = executor.inner.delegation.as_ref() {
        codetether_swarm_delegation::record_subtask_outcome(
            delegation, provider, specialty, success,
        );
    }
}
