//! Construct the default objective and approval-pending run.
use crate::{PendingOkrApproval, default_relay_okr_template};
use codetether_okr::OkrRun;
use uuid::Uuid;
impl PendingOkrApproval {
    /// Create a new pending approval from a task using the default template.
    /// # Arguments
    /// * `task` - Original relay task.
    /// * `agent_count` - Number of agents to display.
    /// * `model` - Model selected for execution.
    /// # Returns
    /// A default objective with a run submitted for approval.
    /// # Examples
    /// ```
    /// let pending = codetether_relay_okr::PendingOkrApproval::new("task".into(), 2, "model".into());
    /// assert!(pending.draft_note.is_none());
    /// ```
    pub fn new(task: String, agent_count: usize, model: String) -> Self {
        let okr_id = Uuid::new_v4();
        let okr = default_relay_okr_template(okr_id, &task);

        let mut run = OkrRun::new(
            okr_id,
            format!("Run {}", chrono::Local::now().format("%Y-%m-%d %H:%M")),
        );
        let _ = run.submit_for_approval();

        Self {
            okr,
            run,
            draft_note: None,
            task,
            agent_count,
            model,
        }
    }
}
