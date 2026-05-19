use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenExhaustionReport {
    pub subtask_id: String,
    pub tokens_used: usize,
    pub tokens_limit: usize,
    pub steps_completed: usize,
    pub changed_files: Vec<String>,
    pub last_tool_calls: Vec<String>,
    pub errors_encountered: Vec<String>,
    pub partial_output: String,
    pub recovery_hint: String,
}

impl TokenExhaustionReport {
    pub fn to_error_message(&self) -> String {
        format!(
            "Sub-agent token exhaustion: subtask={} used={}/{} tokens, \
             steps={}, files_changed={}, recovery: {}",
            self.subtask_id,
            self.tokens_used,
            self.tokens_limit,
            self.steps_completed,
            self.changed_files.len(),
            self.recovery_hint,
        )
    }
}
