//! Create the initial persistent run record without blocking execution.
use super::*;
impl RalphLoop {
    pub(super) fn create_initial_run_record(&self) {
        // Create run record in store
        if let Some(ref store) = self.store {
            let initial_state = RalphRunState {
                run_id: self.run_id.clone(),
                okr_id: None,
                prd: self.state.prd.clone(),
                config: self.config.clone(),
                status: RalphStatus::Running,
                current_iteration: 0,
                max_iterations: self.state.max_iterations,
                progress_log: Vec::new(),
                story_results: Vec::new(),
                error: None,
                created_at: chrono::Utc::now().to_rfc3339(),
                started_at: Some(chrono::Utc::now().to_rfc3339()),
                completed_at: None,
            };
            let s = store.clone();
            self.store_fire_and_forget(async move { s.create_run(&initial_state).await });
        }
    }
}
