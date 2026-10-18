//! Configure and register one successfully created specialist.
use crate::{Host, Session, state::Runtime};
use codetether_autochat_core::transport::attach_handoff_receiver;
use codetether_bus_core::relay::RelayAgentProfile;
use codetether_relay_plan::RelayProfile;
impl<H: Host> Runtime<H> {
    pub(super) fn attach_spawn(
        &mut self,
        mut spawned_session: H::Session,
        profile: RelayProfile,
        reason: String,
        model_ref: &str,
        idx: usize,
    ) {
        let spawned_model_ref = self.team.model_rotation.next_model_ref(model_ref);
        spawned_session.configure(
            spawned_model_ref.clone(),
            profile.name.clone(),
            self.bus.clone(),
            profile.instructions.clone(),
        );

        self.relay.register_agents(&[RelayAgentProfile {
            name: profile.name.clone(),
            capabilities: profile.capabilities.clone(),
        }]);
        attach_handoff_receiver(
            &mut self.team.relay_receivers,
            self.bus.clone(),
            &profile.name,
        );

        self.team
            .ordered_agents
            .insert(idx + 1, profile.name.clone());
        self.team
            .agent_models
            .insert(profile.name.clone(), spawned_model_ref);
        self.team
            .sessions
            .insert(profile.name.clone(), spawned_session);
        self.dynamic_spawn_count += 1;

        tracing::info!(
            agent = %profile.name,
            reason = %reason,
            "Dynamic relay spawn accepted"
        );
    }
}
