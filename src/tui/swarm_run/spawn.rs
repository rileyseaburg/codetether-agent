//! Spawns the parallel swarm executor task launched by `/swarm <task>`.

use crate::swarm::{DecompositionStrategy, SwarmConfig, SwarmControl, SwarmExecutor};
use crate::tui::swarm_view::SwarmEvent;

/// Spawn a background task that runs the swarm and streams events to the TUI.
///
/// Returns a [`SwarmControl`] handle so the caller (TUI) can cancel or
/// pause/resume the running swarm. The executor is attached to the
/// process-wide [`AgentBus`](crate::bus::AgentBus) when one is installed.
pub(super) fn spawn_swarm_run(
    task: String,
    model: Option<String>,
    event_tx: tokio::sync::mpsc::Sender<SwarmEvent>,
) -> SwarmControl {
    let control = SwarmControl::new();
    let run_control = control.clone();
    tokio::spawn(async move {
        let mut executor = SwarmExecutor::new(SwarmConfig {
            model,
            // Cranked for paid tiers: more in-flight requests, no stagger
            // delay, and a tighter retry ceiling so transient rate limits
            // don't stall the whole swarm for 30s.
            max_concurrent_requests: 24,
            request_delay_ms: 0,
            max_delay_ms: 5_000,
            ..Default::default()
        })
        .with_event_tx(event_tx.clone())
        .with_control(run_control);

        if let Some(bus) = crate::bus::global::global() {
            executor = executor.with_bus(bus);
        }

        if let Err(err) = executor
            .execute(&task, DecompositionStrategy::Automatic)
            .await
        {
            let _ = event_tx
                .send(SwarmEvent::Error(format!("Swarm execution failed: {err}")))
                .await;
        }
    });
    control
}
