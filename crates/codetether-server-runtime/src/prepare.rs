//! Ordered startup preparation before A2A and HTTP serving.
use crate::{Host, cognition, services::Services, systems};
use std::time::Instant;

pub(crate) async fn prepare<H: Host>(t0: Instant) -> anyhow::Result<Services<H::Runtime>> {
    let (config, cognition, plugin_registry) = cognition::prepare::<H>(t0).await?;
    let (audit_log, k8s, auth_state, bus) = systems::prepare(t0).await;
    cognition::autostart::<H>(&cognition, t0).await;
    Ok(Services {
        config,
        cognition,
        audit_log,
        k8s,
        auth_state,
        bus,
        plugin_registry,
    })
}
