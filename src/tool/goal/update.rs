//! `update_goal` schema and tool dispatch.

use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

pub(super) struct UpdateGoalTool;

/// Arguments for `update_goal`, including runtime-injected context.
#[derive(Deserialize)]
pub(super) struct Args {
    /// Requested terminal status: `complete` or `blocked`.
    pub status: String,
    /// Worker's account of why the claim holds, handed to the verifier.
    #[serde(default)]
    pub evidence: String,
    #[serde(default, rename = "__ct_session_id")]
    pub session_id: Option<String>,
    #[serde(default, rename = "__ct_current_model")]
    pub current_model: Option<String>,
    #[serde(default, rename = "__ct_parent_workspace")]
    pub workspace: Option<String>,
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
        "Request a terminal goal status. A separate verifier LLM independently re-checks the workspace against the goal and decides; the status changes only if it returns PASS, otherwise its findings come back and the goal stays active. Provide concrete evidence: files, commands run, and their output."
    }
    fn parameters(&self) -> Value {
        json!({"type":"object","properties":{
            "status":{"type":"string","enum":["complete","blocked"]},
            "evidence":{"type":"string","description":"Per-requirement proof: files changed, commands run, and observed results; for blocked, the exact external blocker."}
        },"required":["status","evidence"]})
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        super::update_run::run(serde_json::from_value(input)?).await
    }
}
