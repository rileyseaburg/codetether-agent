//! update_status persistence operation.
use super::*;
impl InMemoryStore {
    pub(super) async fn update_status_impl(
        &self,
        run_id: &str,
        status: RalphStatus,
    ) -> anyhow::Result<()> {
        let mut runs = self.runs.write().await;
        if let Some(run) = runs.get_mut(run_id) {
            run.status = status;
            if status == RalphStatus::Running && run.started_at.is_none() {
                run.started_at = Some(chrono::Utc::now().to_rfc3339());
            }
        }
        Ok(())
    }
}
