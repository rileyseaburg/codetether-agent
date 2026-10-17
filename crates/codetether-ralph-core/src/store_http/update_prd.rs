//! update_prd persistence operation.
use super::*;
impl HttpStore {
    pub(super) async fn update_prd_impl(&self, run_id: &str, prd: &Prd) -> anyhow::Result<()> {
        let url = format!("{}/v1/ralph/runs/{}", self.base_url, run_id);
        let body = json!({ "prd": prd });

        let req = self.apply_auth(self.client.put(&url)).json(&body);
        if let Err(e) = req.send().await {
            warn!(run_id = %run_id, error = %e, "HTTP store update_prd error");
        }
        Ok(())
    }
}
