//! Persist and announce the terminal run state.
use super::*;
impl RalphLoop {
    pub(super) fn finish_run(&self) -> anyhow::Result<RalphState> {
        // Persist final state to store
        if let Some(ref store) = self.store {
            let s = store.clone();
            let rid = self.run_id.clone();
            let status = self.state.status;
            let prd = self.state.prd.clone();
            self.store_fire_and_forget(async move {
                s.update_prd(&rid, &prd).await?;
                s.complete_run(&rid, status).await
            });
        }

        info!(
            "Ralph finished: {:?}, {}/{} stories passed",
            self.state.status,
            self.state.prd.passed_count(),
            self.state.prd.user_stories.len()
        );

        // Emit Complete event
        self.try_send_event(RalphEvent::Complete {
            status: format!("{:?}", self.state.status),
            passed: self.state.prd.passed_count(),
            total: self.state.prd.user_stories.len(),
        });

        Ok(self.state.clone())
    }
}
