//! Runtime effects for shared task handling.
use super::*;
#[async_trait::async_trait]
impl TaskHandlerHost for ApplicationHost<'_> {
    async fn sync_timeline(&self, timeline: &TaskTimeline) {
        sync_timeline_to_runtime(timeline, self.0).await;
    }
    async fn claim(
        &self,
        task_id: &str,
        timeline: &mut TaskTimeline,
    ) -> Result<Option<ClaimedTaskData>> {
        claim_task(self.0, task_id, timeline).await
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
        execute_claimed_task(
            self.0,
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
        task_handler_release::release_handled_task(self.0, task_id, outcome, timeline).await
    }
}
