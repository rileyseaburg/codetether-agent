//! Persistence and events for newly finished Kubernetes results.

use super::{cache, events, state::State};
use crate::swarm::SubTaskResult;

pub(super) async fn results(state: &mut State<'_>, finished: Vec<SubTaskResult>) {
    for result in finished {
        if result.success {
            state
                .completed
                .write()
                .await
                .insert(result.subtask_id.clone(), result.result.clone());
        }
        cache::put(state, &result).await;
        events::finished(state, &result);
        state.results.push(result);
    }
}
