//! Launch of pending subtasks into available Kubernetes pod slots.

use super::super::kubernetes_executor::encode_payload;
use super::{
    cache, events, failure, payload, spec,
    state::{ActiveBranch, State},
};
use std::time::Instant;

pub(super) async fn available(state: &mut State<'_>) {
    while state.active.len() < state.budget {
        let Some(task) = state.pending.pop_front() else {
            break;
        };
        if let Some(result) = cache::find(state, &task).await {
            state.results.push(result);
            continue;
        }
        let encoded = match encode_payload(&payload::build(state, &task).await) {
            Ok(payload) => payload,
            Err(error) => {
                let message = format!("Failed to encode remote payload: {error}");
                events::failed(state, &task, "k8s-encoder", &message);
                state.results.push(failure::result(&task.id, message, 0));
                continue;
            }
        };
        let pod = spec::build(
            encoded,
            state.swarm_id,
            task.stage,
            state.executor.config.k8s_subagent_image.clone(),
        );
        if let Err(error) = state.k8s.spawn_subagent_pod_with_spec(&task.id, pod).await {
            let message = format!("Failed to spawn Kubernetes pod: {error}");
            events::failed(state, &task, "k8s-spawn", &message);
            state.results.push(failure::result(&task.id, message, 0));
            continue;
        }
        let branch = crate::k8s::K8sManager::subagent_pod_name(&task.id);
        state.names.insert(task.id.clone(), task.name.clone());
        state.active.insert(
            task.id.clone(),
            ActiveBranch {
                branch: branch.clone(),
                started_at: Instant::now(),
            },
        );
        events::started(state, &task, &branch);
        tracing::info!(subtask_id = %task.id, pod = %branch, "Spawned Kubernetes sub-agent pod");
    }
}
