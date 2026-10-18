//! Spawns the parallel swarm executor task launched by `/swarm <task>`.

use super::Host;
use codetether_session_control::ControlHandle as SwarmControl;
use codetether_swarm_core::{DecompositionStrategy, SwarmConfig, SwarmEvent};

/// Spawn a background task that runs the swarm and streams events to the TUI.
///
/// Returns a [`SwarmControl`] handle so the caller (TUI) can cancel or
/// pause/resume the running swarm. The executor is attached to the
/// process-wide application event bus when one is installed.
pub(super) fn spawn_swarm_run<H: Host>(
    task: String,
    model: Option<String>,
    event_tx: tokio::sync::mpsc::Sender<SwarmEvent>,
) -> SwarmControl {
    let control = SwarmControl::new();
    let run_control = control.clone();
    tokio::spawn(async move {
        let config = SwarmConfig {
            model,
            // Cranked for paid tiers: more in-flight requests, no stagger
            // delay, and a tighter retry ceiling so transient rate limits
            // don't stall the whole swarm for 30s.
            max_concurrent_requests: 24,
            request_delay_ms: 0,
            max_delay_ms: 5_000,
            ..Default::default()
        };

        if let Err(err) = H::execute(
            config,
            run_control,
            event_tx.clone(),
            task,
            DecompositionStrategy::Automatic,
        )
        .await
        {
            let _ = event_tx
                .send(SwarmEvent::Error(format!("Swarm execution failed: {err}")))
                .await;
        }
    });
    control
}
