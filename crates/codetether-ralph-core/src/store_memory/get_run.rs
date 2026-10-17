//! get_run persistence operation.
use super::*;
impl InMemoryStore {
    pub(super) async fn get_run_impl(&self, run_id: &str) -> anyhow::Result<Option<RalphRunState>> {
        let runs = self.runs.read().await;
        Ok(runs.get(run_id).cloned())
    }
}
