//! Native task execution for shared worker admission and slot cleanup.
use super::super::{WorkerTaskRuntime, handle_task};

pub(super) struct Native;
impl codetether_a2a_worker_core::dispatch::incoming::Handler for Native {
    type Client = reqwest::Client;
    type Bus = crate::bus::AgentBus;
    async fn handle(runtime: &WorkerTaskRuntime, task: &serde_json::Value) -> anyhow::Result<()> {
        handle_task(runtime, task).await
    }
}
