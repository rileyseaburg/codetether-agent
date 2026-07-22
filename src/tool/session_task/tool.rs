//! `session_task` [`Tool`] implementation — the agent-facing entrypoint.

use super::params::Params;
use crate::session::tasks::TaskLog;
use crate::tool::{Tool, ToolResult};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use serde_json::Value;

pub struct SessionTaskTool;

impl Default for SessionTaskTool {
    fn default() -> Self {
        Self
    }
}

impl SessionTaskTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for SessionTaskTool {
    fn id(&self) -> &str {
        "session_task"
    }
    fn name(&self) -> &str {
        "Session Task & Goal"
    }
    fn description(&self) -> &str {
        "Manage the session's goal and task list. Actions: \
         `set_goal` (objective, success_criteria?, forbidden?), \
         `reaffirm` (progress_note), \
         `clear_goal` (reason?), \
         `task_add` (content, id?, parent_id?), \
         `task_status` (id, status: pending|in_progress|done|blocked|cancelled, note?), \
         `list`. Events are appended to the session's .tasks.jsonl."
    }
    fn parameters(&self) -> Value {
        super::schema::value()
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params)?;
        let sid = p
            .ct_session_id
            .clone()
            .or_else(|| std::env::var("CODETETHER_SESSION_ID").ok())
            .ok_or_else(|| anyhow!("session id not available; cannot locate task log"))?;
        let log = TaskLog::for_session(&sid)?;
        super::dispatch::run(&log, p).await
    }
}
