//! set_error persistence operation.
use super::*;
impl HttpStore {
    pub(super) async fn set_error_impl(&self, run_id: &str, error: &str) -> anyhow::Result<()> {
        let url = format!("{}/v1/ralph/runs/{}", self.base_url, run_id);
        let body = json!({ "error": error });

        let req = self.apply_auth(self.client.put(&url)).json(&body);
        if let Err(e) = req.send().await {
            warn!(run_id = %run_id, error = %e, "HTTP store set_error error");
        }
        Ok(())
    }
}
