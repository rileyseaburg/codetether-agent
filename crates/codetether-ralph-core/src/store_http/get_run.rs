//! get_run persistence operation.
use super::*;
impl HttpStore {
    pub(super) async fn get_run_impl(&self, run_id: &str) -> anyhow::Result<Option<RalphRunState>> {
        let url = format!("{}/v1/ralph/runs/{}", self.base_url, run_id);
        let req = self.apply_auth(self.client.get(&url));

        match req.send().await {
            Ok(resp) if resp.status().is_success() => match resp.json::<RalphRunState>().await {
                Ok(state) => Ok(Some(state)),
                Err(e) => {
                    warn!(run_id = %run_id, error = %e, "HTTP store get_run parse error");
                    Ok(None)
                }
            },
            Ok(resp) if resp.status().as_u16() == 404 => Ok(None),
            Ok(resp) => {
                warn!(run_id = %run_id, status = %resp.status(), "HTTP store get_run failed");
                Ok(None)
            }
            Err(e) => {
                warn!(run_id = %run_id, error = %e, "HTTP store get_run error");
                Ok(None)
            }
        }
    }
}
