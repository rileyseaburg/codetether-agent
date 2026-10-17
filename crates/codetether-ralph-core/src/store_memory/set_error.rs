//! set_error persistence operation.
use super::*;
impl InMemoryStore {
    pub(super) async fn set_error_impl(&self, run_id: &str, error: &str) -> anyhow::Result<()> {
        let mut runs = self.runs.write().await;
        if let Some(run) = runs.get_mut(run_id) {
            run.error = Some(error.to_string());
        }
        Ok(())
    }
}
