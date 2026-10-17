//! create_run persistence operation.
use super::*;
impl InMemoryStore {
    pub(super) async fn create_run_impl(&self, state: &RalphRunState) -> anyhow::Result<()> {
        let mut runs = self.runs.write().await;
        runs.insert(state.run_id.clone(), state.clone());
        Ok(())
    }
}
