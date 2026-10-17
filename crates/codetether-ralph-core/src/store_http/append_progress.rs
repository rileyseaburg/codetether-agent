//! append_progress persistence operation.
use super::*;
impl HttpStore {
    pub(super) async fn append_progress_impl(
        &self,
        run_id: &str,
        entry: &ProgressEntry,
    ) -> anyhow::Result<()> {
        // Append progress as a log entry via the update endpoint
        let url = format!("{}/v1/ralph/runs/{}", self.base_url, run_id);
        let body = json!({
            "logs": [entry],
        });

        let req = self.apply_auth(self.client.put(&url)).json(&body);
        if let Err(e) = req.send().await {
            warn!(run_id = %run_id, error = %e, "HTTP store append_progress error");
        }
        Ok(())
    }
}
