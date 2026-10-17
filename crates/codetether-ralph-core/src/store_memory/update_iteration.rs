//! update_iteration persistence operation.
use super::*;
impl InMemoryStore {
    pub(super) async fn update_iteration_impl(
        &self,
        run_id: &str,
        iteration: usize,
    ) -> anyhow::Result<()> {
        let mut runs = self.runs.write().await;
        if let Some(run) = runs.get_mut(run_id) {
            run.current_iteration = iteration;
        }
        Ok(())
    }
}
