//! Interactive Ralph launch behavior.
use super::Host;
use codetether_ralph_core::{RalphConfig, events::RalphEvent};
use std::sync::Arc;

/// Spawn a background task that drives the native Ralph loop to completion and
/// streams events into the TUI through `event_tx`. The sender is dropped on
/// task exit, which the TUI notices via a disconnected `try_recv` and uses
/// to detach the receiver.
/// # Arguments
/// * `prd_path` - PRD file to execute.
/// * `provider` - Selected model provider.
/// * `model` - Provider-local model name.
/// * `max_iters` - Maximum loop iterations.
/// * `event_tx` - Event channel attached to the interactive view.
/// # Returns
/// Immediately after spawning the detached run; failures are sent as events.
/// # Examples
/// ~~~no_run
/// # fn example<H: codetether_ralph_cli::interactive::Host>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>, tx: tokio::sync::mpsc::Sender<codetether_ralph_core::events::RalphEvent>) {
/// codetether_ralph_cli::interactive::spawn_ralph_run::<H>("prd.json".into(), provider, "model".into(), 10, tx);
/// # }
/// ~~~
pub fn spawn_ralph_run<H: Host>(
    prd_path: std::path::PathBuf,
    provider: Arc<dyn codetether_provider_core::Provider>,
    model: String,
    max_iters: usize,
    event_tx: tokio::sync::mpsc::Sender<RalphEvent>,
) {
    tokio::spawn(async move {
        let config = RalphConfig {
            prd_path: prd_path.to_string_lossy().to_string(),
            max_iterations: max_iters,
            model: Some(model.clone()),
            ..Default::default()
        };

        let mut ralph =
            match H::create(prd_path.clone(), provider, model, config, event_tx.clone()).await {
                Ok(r) => r,
                Err(err) => {
                    let _ = event_tx
                        .send(RalphEvent::Error(format!(
                            "Failed to initialise Ralph: {err}"
                        )))
                        .await;
                    return;
                }
            };

        if let Err(err) = H::run(&mut ralph).await {
            let _ = event_tx
                .send(RalphEvent::Error(format!("Ralph loop errored: {err}")))
                .await;
        }
    });
}
