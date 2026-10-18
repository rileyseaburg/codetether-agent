//! Initialize and register the ordered relay team.
use super::*;
pub(crate) async fn prepare_relay_team<H: Host>(
    registry: &Arc<H::Registry>,
    model: &str,
    profiles: &[(String, String, Vec<String>)],
    relay_bus: &Arc<AgentBus>,
    delegation: Option<&DelegationState>,
    relay: &ProtocolRelayRuntime,
    story_id: &str,
    max_rounds: usize,
) -> anyhow::Result<PreparedRelayTeam<H>> {
    let mut sessions: HashMap<String, H::Session> = HashMap::new();
    let mut ordered_agents: Vec<String> = Vec::new();
    let mut relay_profiles: Vec<RelayAgentProfile> = Vec::new();

    for (name, instructions, capabilities) in profiles {
        let session =
            create_relay_session::<H>(registry, model, name, instructions, relay_bus, delegation)
                .await?;

        relay_profiles.push(RelayAgentProfile {
            name: name.clone(),
            capabilities: capabilities.clone(),
        });
        ordered_agents.push(name.clone());
        sessions.insert(name.clone(), session);
    }

    relay.register_agents(&relay_profiles);

    info!(target: "codetether_agent::ralph::ralph_loop::relay_prepare",
        story_id = %story_id,
        agents = ordered_agents.len(),
        rounds = max_rounds,
        "Starting relay team for story"
    );

    Ok(PreparedRelayTeam {
        sessions,
        ordered_agents,
        _relay_profiles: relay_profiles,
    })
}
