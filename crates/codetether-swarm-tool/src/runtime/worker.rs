//! Acquire the concurrency permit before running a prepared worker.
use super::{Host, WorkerContext};
use crate::{model_selection::ModelSelection, task_input::TaskInput, task_result::TaskResult};
use anyhow::Result;
use codetether_swarm_core::{SwarmEvent, result_store::ResultStore};
use std::sync::Arc;
use tokio::sync::{Semaphore, mpsc};

pub(super) async fn run<H: Host>(
    task: TaskInput,
    context: Result<WorkerContext>,
    model_selection: ModelSelection,
    store: Arc<ResultStore>,
    semaphore: Arc<Semaphore>,
    events: mpsc::Sender<SwarmEvent>,
    max_steps: usize,
    timeout_secs: u64,
) -> Result<TaskResult> {
    let _permit = semaphore
        .acquire()
        .await
        .expect("swarm semaphore closed unexpectedly");
    let context = context?;
    let task_id = context.task_id.clone();
    let output =
        super::invocation::run::<H>(&task, context, store, events, max_steps, timeout_secs).await?;
    Ok(super::outcome::finish(
        task,
        task_id,
        model_selection,
        output,
    ))
}
