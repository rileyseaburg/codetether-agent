//! Construct and attach one initially planned specialist.
use crate::{Session, team::Team};
use codetether_autochat_core::transport::attach_handoff_receiver;
use codetether_bus_core::AgentBus;
use codetether_relay_plan::RelayProfile;
use std::sync::Arc;
impl<S: Session> Team<S> {
    pub(super) async fn add_initial(
        &mut self,
        profile: &RelayProfile,
        model_ref: &str,
        bus: &Arc<AgentBus>,
    ) -> anyhow::Result<()> {
        let assigned_model_ref = self.model_rotation.next_model_ref(model_ref);
        let mut session = S::new().await?;
        session.configure(
            assigned_model_ref.clone(),
            profile.name.clone(),
            bus.clone(),
            profile.instructions.clone(),
        );
        self.agent_models
            .insert(profile.name.clone(), assigned_model_ref);
        self.sessions.insert(profile.name.clone(), session);
        attach_handoff_receiver(&mut self.relay_receivers, bus.clone(), &profile.name);

        Ok(())
    }
}
