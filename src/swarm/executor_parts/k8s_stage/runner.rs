//! Lifecycle driver for a Kubernetes-backed stage.

use super::super::SwarmExecutor;
use crate::k8s::K8sManager;
use crate::swarm::{Orchestrator, SubTask, SubTaskResult};
use anyhow::{Result, bail};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tokio::time::{Duration, MissedTickBehavior};

pub(in crate::swarm::executor) async fn execute(
    executor: &SwarmExecutor,
    orchestrator: &Orchestrator,
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
        orchestrator.provider().into(),
        orchestrator.model().into(),
        tasks,
        completed,
    );
    let mut tick = tokio::time::interval(Duration::from_secs(5));
    tick.set_missed_tick_behavior(MissedTickBehavior::Skip);
    tick.tick().await;
    loop {
        super::spawn::available(&mut state).await;
        if state.pending.is_empty() && state.active.is_empty() {
            break;
        }
        tick.tick().await;
        let finished = super::poll::finished(&mut state).await;
        super::record::results(&mut state, finished).await;
        super::collapse::sample(&mut state).await;
    }
    super::cleanup::finish(&mut state).await;
    Ok(state.results)
}
