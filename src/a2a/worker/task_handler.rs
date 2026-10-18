//! Application session execution hook for the native worker lifecycle.
use super::{WorkerTaskRuntime, execute_claimed_task};
use codetether_a2a_worker_core::{outcome::TaskInner, task_timeline::TaskTimeline};
use codetether_provenance::ClaimProvenance;

pub(super) struct Native;
impl codetether_native_worker::lifecycle::Executor for Native {
    async fn execute(
        runtime: &WorkerTaskRuntime,
        task: &serde_json::Value,
        task_id: &str,
        title: &str,
        provenance: &ClaimProvenance,
        provider_keys: Option<serde_json::Value>,
        timeline: &mut TaskTimeline,
    ) -> TaskInner {
        execute_claimed_task(
            runtime,
            task,
            task_id,
            title,
            provenance,
            provider_keys,
            timeline,
        )
        .await
    }
}
