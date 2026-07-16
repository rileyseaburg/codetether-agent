//! Terminal direct-swarm events and summary statistics.

use super::{super::task_result::TaskResult, stats};
use crate::tui::swarm_view::SwarmEvent;
use std::time::Duration;
use tokio::sync::mpsc;

pub(super) fn send(events: &mpsc::Sender<SwarmEvent>, results: &[TaskResult], elapsed: Duration) {
    for result in results {
        if !result.output.is_empty() {
            emit(
                events,
                SwarmEvent::AgentOutput {
                    subtask_id: result.task_id.clone(),
                    output: result.output.clone(),
                },
            );
        }
        if let Some(error) = &result.error {
            emit(
                events,
                SwarmEvent::AgentError {
                    subtask_id: result.task_id.clone(),
                    error: error.clone(),
                },
            );
        }
        emit(
            events,
            SwarmEvent::AgentComplete {
                subtask_id: result.task_id.clone(),
                success: result.success,
                steps: result.steps,
            },
        );
    }
    let (success, stats) = stats::build(results, elapsed);
    emit(events, SwarmEvent::Complete { success, stats });
}

fn emit(events: &mpsc::Sender<SwarmEvent>, event: SwarmEvent) {
    let _ = events.try_send(event);
}
