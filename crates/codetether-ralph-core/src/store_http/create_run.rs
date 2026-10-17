//! create_run persistence operation.
use super::*;
impl HttpStore {
    pub(super) async fn create_run_impl(&self, state: &RalphRunState) -> anyhow::Result<()> {
        let url = format!("{}/v1/ralph/runs", self.base_url);
        let body = json!({
            "id": state.run_id,
            "prd": state.prd,
            "workspace_id": state.okr_id,
            "model": state.config.model,
            "status": Self::status_str(state.status),
            "max_iterations": state.max_iterations,
            "run_mode": if state.config.parallel_enabled { "parallel" } else { "sequential" },
            "max_parallel": state.config.max_concurrent_stories,
        });

        let req = self.apply_auth(self.client.post(&url)).json(&body);
        match req.send().await {
            Ok(resp) if resp.status().is_success() => {
                debug!(run_id = %state.run_id, "Created run in HTTP store");
            }
            Ok(resp) => {
                warn!(
                    run_id = %state.run_id,
                    status = %resp.status(),
                    "HTTP store create_run failed"
                );
            }
            Err(e) => {
                warn!(run_id = %state.run_id, error = %e, "HTTP store create_run error");
            }
        }
        Ok(())
    }
}
