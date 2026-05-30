/// Shared progress state readable by the heartbeat task.
#[derive(Debug, Clone, serde::Serialize)]
pub struct TaskProgressState {
    pub task_id: Option<String>,
    pub current_checkpoint: Option<String>,
    pub elapsed_secs: f64,
    pub remaining_secs: f64,
    pub budget_pct_used: f64,
    pub checkpoints_reached: Vec<String>,
    pub last_detail: Option<String>,
}

impl Default for TaskProgressState {
    fn default() -> Self {
        Self {
            task_id: None,
            current_checkpoint: None,
            elapsed_secs: 0.0,
            remaining_secs: 0.0,
            budget_pct_used: 0.0,
            checkpoints_reached: Vec::new(),
            last_detail: None,
        }
    }
}

impl TaskProgressState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn clear(&mut self) {
        self.task_id = None;
        self.current_checkpoint = None;
        self.checkpoints_reached.clear();
        self.last_detail = None;
    }
}
