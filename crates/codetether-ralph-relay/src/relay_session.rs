//! Select a provider and initialize one named relay session.
use super::*;
pub(crate) async fn create_relay_session<H: Host>(
    registry: &Arc<H::Registry>,
    model: &str,
    name: &str,
    instructions: &str,
    relay_bus: &Arc<AgentBus>,
    delegation: Option<&DelegationState>,
) -> anyhow::Result<H::Session> {
    // LCB delegation: pick best provider for this relay role.
    let default_provider = H::providers(registry).first().copied().unwrap_or("openai");
    let (_role_provider, role_model) = match delegation {
        Some(state) => codetether_ralph_core::delegation::choose_provider_for_role(
            state,
            name,
            default_provider,
            model,
            || H::providers(registry),
        ),
        None => (default_provider.to_string(), model.to_string()),
    };
    let role_model_str = role_model.clone();

    H::new_session(role_model_str, name, instructions, relay_bus).await
}
