//! Initialize models and sessions before relay registration.
use crate::{Host, team::Team};
use codetether_autochat_core::model_rotation::{
    RelayModelRotation, build_round_robin_model_rotation,
};
use codetether_bus_core::{AgentBus, relay::RelayAgentProfile};
use codetether_relay_plan::RelayProfile;
use std::{collections::HashMap, sync::Arc};
pub(super) async fn build<H: Host>(
    profiles: Vec<RelayProfile>,
    registry: &Option<Arc<H::Registry>>,
    model_ref: &str,
    bus: &Arc<AgentBus>,
) -> anyhow::Result<(Team<H::Session>, Vec<RelayAgentProfile>)> {
    let relay_profiles = crate::team_registration::registrations(&profiles);

    let ordered_agents: Vec<String> = profiles
        .iter()
        .map(|profile| profile.name.clone())
        .collect();
    let sessions: HashMap<String, H::Session> = HashMap::new();
    let relay_receivers: HashMap<String, codetether_bus_core::BusHandle> = HashMap::new();
    let agent_models: HashMap<String, String> = HashMap::new();
    let model_rotation = if let Some(registry) = registry {
        build_round_robin_model_rotation(registry.as_ref(), model_ref).await
    } else {
        RelayModelRotation::fallback(model_ref)
    };

    let mut team = Team {
        ordered_agents,
        sessions,
        relay_receivers,
        agent_models,
        model_rotation,
    };
    for profile in &profiles {
        team.add_initial(profile, model_ref, bus).await?;
    }

    if team.ordered_agents.len() < 2 {
        anyhow::bail!("Autochat needs at least 2 agents to relay.");
    }

    Ok((team, relay_profiles))
}
