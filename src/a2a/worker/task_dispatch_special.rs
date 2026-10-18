//! Application forage hook for native special-task dispatch.
use super::{TaskContext, WorkerTaskRuntime};
use anyhow::Result;
use serde_json::{Map, Value};
struct Forage;
impl codetether_native_worker::special::Forage for Forage {
    async fn execute(
        title: &str,
        prompt: &str,
        metadata: &Map<String, Value>,
        selected_model: Option<String>,
    ) -> Result<String> {
        codetether_forage_runtime::worker_task::handle::<crate::forage::execution_host::Native>(
            title,
            prompt,
            metadata,
            selected_model,
        )
        .await
    }
}
pub(super) async fn dispatch_special_task(
    runtime: &WorkerTaskRuntime,
    task: &Value,
    title: &str,
    context: &TaskContext,
) -> Option<codetether_a2a_worker_core::outcome::TaskInner> {
    codetether_native_worker::special::dispatch_special_task::<Forage>(
        runtime, task, title, context,
    )
    .await
}
