//! list_runs persistence operation.
use super::*;
impl HttpStore {
    pub(super) async fn list_runs_impl(&self) -> anyhow::Result<Vec<RalphRunSummary>> {
        let url = format!("{}/v1/ralph/runs", self.base_url);
        let req = self.apply_auth(self.client.get(&url));

        match req.send().await {
            Ok(resp) if resp.status().is_success() => {
                match resp.json::<Vec<RalphRunSummary>>().await {
                    Ok(runs) => Ok(runs),
                    Err(e) => {
                        warn!(error = %e, "HTTP store list_runs parse error");
                        Ok(Vec::new())
                    }
                }
            }
            Ok(resp) => {
                warn!(status = %resp.status(), "HTTP store list_runs failed");
                Ok(Vec::new())
            }
            Err(e) => {
                warn!(error = %e, "HTTP store list_runs error");
                Ok(Vec::new())
            }
        }
    }
}
