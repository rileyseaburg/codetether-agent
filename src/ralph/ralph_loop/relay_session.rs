//! Select a provider and initialize one named relay session.
use super::*;
impl RalphLoop {
    pub(super) async fn create_relay_session(
        registry: &Arc<ProviderRegistry>,
        model: &str,
        name: &str,
        instructions: &str,
        relay_bus: &Arc<AgentBus>,
        delegation: Option<&DelegationState>,
    ) -> anyhow::Result<Session> {
        // LCB delegation: pick best provider for this relay role.
        let default_provider = registry.list().first().copied().unwrap_or("openai");
        let (_role_provider, role_model) = match delegation {
            Some(state) => crate::ralph::delegation::choose_provider_for_role(
                registry,
                state,
                name,
                default_provider,
                model,
            ),
            None => (default_provider.to_string(), model.to_string()),
        };
        let role_model_str = role_model.clone();

        let mut session = Session::new().await?;
        session.metadata.model = Some(role_model_str);
        session.set_agent_name(name.to_string());
        session.bus = Some(relay_bus.clone());
        session.add_message(Message {
            role: Role::System,
            content: vec![ContentPart::Text {
                text: instructions.to_string(),
            }],
        });

        Ok(session)
    }
}
