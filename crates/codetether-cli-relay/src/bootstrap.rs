//! Start services and register the complete initial team.
use crate::{Host, initial::Initial, state::Runtime};
use codetether_autochat_core::shared_context::SharedRelayContext;
use codetether_bus_core::{AgentBus, relay::ProtocolRelayRuntime};
use uuid::Uuid;
pub(super) async fn start<H: Host>(
    agent_count: usize,
    task: &str,
    model_ref: &str,
    okr_id: Option<Uuid>,
    okr_run_id: Option<Uuid>,
) -> anyhow::Result<Runtime<H>> {
    let bus = AgentBus::new().into_arc();
    H::spawn_bus_sink(bus.clone());
    let relay = ProtocolRelayRuntime::new(bus.clone());
    let registry = H::load_registry().await.ok();
    let (profiles, planner_used) =
        crate::profiles::select::<H>(&registry, task, model_ref, agent_count).await;
    let (team, relay_profiles) =
        crate::team_build::build::<H>(profiles, &registry, model_ref, &bus).await?;
    relay.register_agents(&relay_profiles);
    let context_receiver = bus.handle(format!("relay-context-{}", relay.relay_id()));
    let shared_context = SharedRelayContext::default();
    let kr_targets = codetether_relay_okr::progress::targets(okr_id, okr_run_id).await;
    Ok(Runtime::assemble(
        Initial {
            bus,
            relay,
            registry,
            team,
            context_receiver,
            shared_context,
            kr_targets,
            planner_used,
        },
        task,
    ))
}
