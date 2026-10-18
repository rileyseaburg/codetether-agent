//! Runtime effects for shared task handling.
use super::*;
#[async_trait::async_trait]
impl<E: Executor> TaskHandlerHost for ApplicationHost<'_, E> {
    async fn sync_timeline(&self, timeline: &TaskTimeline) {
        sync_timeline_to_runtime(timeline, self.runtime).await;
    }
    async fn claim(
        &self,
        task_id: &str,
        timeline: &mut TaskTimeline,
    ) -> Result<Option<ClaimedTaskData>> {
        claim_task(self.runtime, task_id, timeline).await
    }
    async fn execute(
        &self,
        task: &serde_json::Value,
        task_id: &str,
        title: &str,
        provenance: &ClaimProvenance,
        provider_keys: Option<serde_json::Value>,
        timeline: &mut TaskTimeline,
    ) -> TaskInner {
        E::execute(
            self.runtime,
            task,
            task_id,
            title,
            provenance,
            provider_keys,
            timeline,
        )
        .await
    }
    async fn release(
        &self,
        task_id: &str,
        outcome: TaskOutcome,
        timeline: &mut TaskTimeline,
    ) -> Result<()> {
        release::release_handled_task(self.runtime, task_id, outcome, timeline).await
    }
}
