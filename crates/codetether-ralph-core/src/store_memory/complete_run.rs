//! complete_run persistence operation.
use super::*;
impl InMemoryStore {
    pub(super) async fn complete_run_impl(
        &self,
        run_id: &str,
        status: RalphStatus,
    ) -> anyhow::Result<()> {
        let mut runs = self.runs.write().await;
        if let Some(run) = runs.get_mut(run_id) {
            run.status = status;
            run.completed_at = Some(chrono::Utc::now().to_rfc3339());
        }
        Ok(())
    }
}
