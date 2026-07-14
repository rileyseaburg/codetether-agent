//! Publication of successful stage results.

use super::state::Run;
use crate::swarm::SubTaskResult;

pub(super) async fn publish(run: &Run<'_>, stage: usize, results: &[SubTaskResult]) {
    let mut completed = run.completed.write().await;
    for result in results.iter().filter(|result| result.success) {
        completed.insert(result.subtask_id.clone(), result.result.clone());
        let tags = vec![
            format!("stage:{stage}"),
            format!("subtask:{}", result.subtask_id),
        ];
        let _ = run
            .executor
            .result_store
            .publish(
                &result.subtask_id,
                &result.subagent_id,
                &result.result,
                tags,
                None,
            )
            .await;
    }
}
