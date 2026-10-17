//! append_progress persistence operation.
use super::*;
impl InMemoryStore {
    pub(super) async fn append_progress_impl(
        &self,
        run_id: &str,
        entry: &ProgressEntry,
    ) -> anyhow::Result<()> {
        let mut runs = self.runs.write().await;
        if let Some(run) = runs.get_mut(run_id) {
            run.progress_log.push(entry.clone());
        }
        Ok(())
    }
}
