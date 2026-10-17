//! Adapt the public persistence trait to focused operation modules.
use super::*;
#[async_trait]
impl RalphStateStore for InMemoryStore {
    async fn create_run(&self, state: &RalphRunState) -> Result<()> {
        self.create_run_impl(state).await
    }
    async fn update_status(&self, run_id: &str, status: RalphStatus) -> Result<()> {
        self.update_status_impl(run_id, status).await
    }
    async fn update_iteration(&self, run_id: &str, iteration: usize) -> Result<()> {
        self.update_iteration_impl(run_id, iteration).await
    }
    async fn record_story_result(&self, run_id: &str, result: &StoryResultEntry) -> Result<()> {
        self.record_story_result_impl(run_id, result).await
    }
    async fn append_progress(&self, run_id: &str, entry: &ProgressEntry) -> Result<()> {
        self.append_progress_impl(run_id, entry).await
    }
    async fn update_prd(&self, run_id: &str, prd: &Prd) -> Result<()> {
        self.update_prd_impl(run_id, prd).await
    }
    async fn set_error(&self, run_id: &str, error: &str) -> Result<()> {
        self.set_error_impl(run_id, error).await
    }
    async fn complete_run(&self, run_id: &str, status: RalphStatus) -> Result<()> {
        self.complete_run_impl(run_id, status).await
    }
    async fn get_run(&self, run_id: &str) -> Result<Option<RalphRunState>> {
        self.get_run_impl(run_id).await
    }
    async fn list_runs(&self) -> Result<Vec<RalphRunSummary>> {
        self.list_runs_impl().await
    }
}
