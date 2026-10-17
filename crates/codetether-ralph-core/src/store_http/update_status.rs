//! update_status persistence operation.
use super::*;
impl HttpStore {
    pub(super) async fn update_status_impl(
        &self,
        run_id: &str,
        status: RalphStatus,
    ) -> anyhow::Result<()> {
        let url = format!("{}/v1/ralph/runs/{}", self.base_url, run_id);
        let mut body = json!({ "status": Self::status_str(status) });
        if status == RalphStatus::Running {
            body["started_at"] = json!(chrono::Utc::now().to_rfc3339());
        }

        let req = self.apply_auth(self.client.put(&url)).json(&body);
        if let Err(e) = req.send().await {
            warn!(run_id = %run_id, error = %e, "HTTP store update_status error");
        }
        Ok(())
    }
}
