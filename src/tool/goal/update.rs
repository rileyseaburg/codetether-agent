//! `update_goal` schema and tool dispatch.

use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

pub(super) struct UpdateGoalTool;

#[derive(Deserialize)]
pub(super) struct Args {
    pub status: String,
    #[serde(default, rename = "__ct_session_id")]
    pub session_id: Option<String>,
}

#[async_trait]
impl Tool for UpdateGoalTool {
    fn id(&self) -> &str {
        "update_goal"
    }
    fn name(&self) -> &str {
        "Update Goal"
    }
    fn description(&self) -> &str {
        "Mark the current goal complete only after its completion audit succeeds, or blocked only after the same blocker prevents progress for three consecutive goal turns."
    }
    fn parameters(&self) -> Value {
        json!({"type":"object","properties":{
        "status":{"type":"string","enum":["complete","blocked"]}
    },"required":["status"]})
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        super::update_run::run(serde_json::from_value(input)?).await
    }
}
