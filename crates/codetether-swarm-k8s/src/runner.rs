//! Lifecycle driver for a Kubernetes-backed stage.

use crate::Host;
use anyhow::{Result, bail};
use codetether_k8s::K8sManager;
use codetether_swarm_core::{SubTask, SubTaskResult};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tokio::time::{Duration, MissedTickBehavior};

/// Execute one Kubernetes-backed dependency stage.
/// # Arguments
/// Receives executor services, selected provider/model, tasks, prior results and swarm ID.
/// # Returns
/// Stage results, ordered with any promoted branch first.
/// # Errors
/// Returns an error when a Kubernetes client is unavailable.
/// # Examples
/// ```no_run
/// async fn stage<H: codetether_swarm_k8s::Host>(host: &H) -> anyhow::Result<()> {
///     let prior = std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new()));
///     codetether_swarm_k8s::execute(host, "provider", "model", Vec::new(), prior, "swarm").await?;
///     Ok(())
/// }
/// ```
pub async fn execute<H: Host>(
    executor: &H,
    provider: &str,
    model: &str,
    tasks: Vec<SubTask>,
    completed: Arc<RwLock<HashMap<String, String>>>,
    swarm: &str,
) -> Result<Vec<SubTaskResult>> {
    let k8s = K8sManager::new().await;
    if !k8s.is_available() {
        bail!("Kubernetes execution requested but no client is available");
    }
    let mut state = super::state::State::new(
        executor,
        swarm,
        k8s,
        provider.into(),
        model.into(),
        tasks,
        completed,
    );
    let mut tick = tokio::time::interval(Duration::from_secs(5));
    tick.set_missed_tick_behavior(MissedTickBehavior::Skip);
    tick.tick().await;
    loop {
        if super::control::stopped(executor.control()) {
            break;
        }
        super::spawn::available(&mut state).await;
        if state.pending.is_empty() && state.active.is_empty() {
            break;
        }
        tokio::select! {
            _ = super::control::cancelled(executor.control()) => break,
            _ = tick.tick() => {},
        }
        let finished = super::poll::finished(&mut state).await;
        super::record::results(&mut state, finished).await;
        super::collapse::sample(&mut state).await;
    }
    super::cleanup::finish(&mut state).await;
    Ok(state.results)
}
