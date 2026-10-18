//! Shared admission and slot cleanup connected to native lifecycle execution.
use super::{Executor, TaskDispatcher, handle_task};
use crate::WorkerTaskRuntime;
impl<E: Executor> codetether_a2a_worker_core::dispatch::incoming::Handler for TaskDispatcher<E> {
    type Client = reqwest::Client;
    type Bus = codetether_bus_core::AgentBus;
    async fn handle(runtime: &WorkerTaskRuntime, task: &serde_json::Value) -> anyhow::Result<()> {
        handle_task::<E>(runtime, task).await
    }
}
impl<E: Executor> crate::pending::Dispatcher for TaskDispatcher<E> {
    async fn dispatch(task: &serde_json::Value, runtime: &WorkerTaskRuntime) {
        codetether_a2a_worker_core::dispatch::incoming::spawn_task_handler::<Self>(task, runtime)
            .await;
    }
}
