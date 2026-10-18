//! Model-facing update_goal schema and verified execution entry point.
use super::Host;
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::{Value, json};
use std::marker::PhantomData;
/// Goal-transition request tool using an independent verifier.
/// # Examples
/// ```no_run
/// fn tool<H: codetether_goal_tools::UpdateHost>() {
///     let _tool = codetether_goal_tools::UpdateGoalTool::<H>::new();
/// }
/// ```
pub struct UpdateGoalTool<H>(PhantomData<fn() -> H>);
impl<H> UpdateGoalTool<H> {
    /// Construct the tool without starting verification or touching storage.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}
impl<H> Default for UpdateGoalTool<H> {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<H: Host + 'static> Tool for UpdateGoalTool<H> {
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
        super::run::run::<H>(serde_json::from_value(input)?).await
    }
}
