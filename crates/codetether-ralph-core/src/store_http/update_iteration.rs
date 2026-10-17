//! update_iteration persistence operation.
use super::*;
impl HttpStore {
    pub(super) async fn update_iteration_impl(
        &self,
        run_id: &str,
        iteration: usize,
    ) -> anyhow::Result<()> {
        let url = format!("{}/v1/ralph/runs/{}", self.base_url, run_id);
        let body = json!({ "current_iteration": iteration });

        let req = self.apply_auth(self.client.put(&url)).json(&body);
        if let Err(e) = req.send().await {
            warn!(run_id = %run_id, error = %e, "HTTP store update_iteration error");
        }
        Ok(())
    }
}
