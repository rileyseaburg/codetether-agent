//! update_prd persistence operation.
use super::*;
impl InMemoryStore {
    pub(super) async fn update_prd_impl(&self, run_id: &str, prd: &Prd) -> anyhow::Result<()> {
        let mut runs = self.runs.write().await;
        if let Some(run) = runs.get_mut(run_id) {
            run.prd = prd.clone();
        }
        Ok(())
    }
}
