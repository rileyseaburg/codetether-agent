//! `create_goal` schema and tool dispatch.

use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

pub(super) struct CreateGoalTool;

#[derive(Deserialize)]
pub(super) struct Args {
    pub objective: String,
    #[serde(default, rename = "__ct_session_id")]
    pub session_id: Option<String>,
}

#[async_trait]
impl Tool for CreateGoalTool {
    fn id(&self) -> &str {
        "create_goal"
    }
    fn name(&self) -> &str {
        "Create Goal"
    }
    fn description(&self) -> &str {
        "Create an uncapped goal only when explicitly requested. Fails if an unfinished goal exists; use update_goal only for status. Budgets are controlled only by humans through TUI/authenticated API controls."
    }
    fn parameters(&self) -> Value {
        json!({"type":"object","properties":{
        "objective":{"type":"string"}
    },"required":["objective"]})
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        if let Some(refused) = super::budget_guard::refusal(&input) {
            return Ok(refused);
        }
        super::create_run::run(serde_json::from_value(input)?).await
    }
}
