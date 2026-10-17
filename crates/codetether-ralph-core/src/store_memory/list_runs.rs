//! list_runs persistence operation.
use super::*;
impl InMemoryStore {
    pub(super) async fn list_runs_impl(&self) -> anyhow::Result<Vec<RalphRunSummary>> {
        let runs = self.runs.read().await;
        Ok(runs
            .values()
            .map(|run| RalphRunSummary {
                run_id: run.run_id.clone(),
                okr_id: run.okr_id.clone(),
                status: run.status,
                passed: run.prd.passed_count(),
                total: run.prd.user_stories.len(),
                current_iteration: run.current_iteration,
                created_at: run.created_at.clone(),
            })
            .collect())
    }
}
