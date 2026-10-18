//! Bind shared swarm provider delegation to the native registry.
use crate::{provider::ProviderRegistry, session::delegation::DelegationState};
/// Pick a provider with the shared specialty-ranking policy.
pub fn choose_provider_for_subtask(
    registry: &ProviderRegistry,
    state: &DelegationState,
    specialty: &str,
    default_provider: &str,
    default_model: &str,
) -> (String, String) {
    codetether_swarm_delegation::choose_provider_for_subtask(
        || registry.list(),
        state,
        specialty,
        default_provider,
        default_model,
    )
}
