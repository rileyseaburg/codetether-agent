//! Session task tool identity, schema and journal dispatch.
use crate::{Host, SessionTaskTool, params::Params};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use codetether_goal_core::TaskLog;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;

#[async_trait]
impl<H: Host> Tool for SessionTaskTool<H> {
    fn id(&self) -> &str {
        "session_task"
    }
    fn name(&self) -> &str {
        "Session Tasks"
    }
    fn description(&self) -> &str {
        "Manage work items, not the session goal. Use create_goal/get_goal/edit_goal/update_goal \
         for the overall objective and completion criteria. Finishing tasks does not complete a goal. Actions: \
         `task_add` (content, id?, parent_id?), \
         `task_status` (id, status: pending|in_progress|done|blocked|cancelled, note?), \
         `list` (tasks only). Events are appended to the session's .tasks.jsonl."
    }
    fn parameters(&self) -> Value {
        crate::schema::value()
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        if let Some(refused) = crate::budget_refusal(&params) {
            return Ok(refused);
        }
        let p: Params = serde_json::from_value(params)?;
        let sid = p
            .ct_session_id
            .clone()
            .or_else(|| std::env::var("CODETETHER_SESSION_ID").ok())
            .ok_or_else(|| anyhow!("session id not available; cannot locate task log"))?;
        let log = TaskLog::at(H::log_path(&sid)?);
        crate::dispatch::run::<H>(&log, p).await
    }
}
