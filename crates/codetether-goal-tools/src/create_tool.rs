//! `create_goal` schema and tool dispatch.

use crate::Host;
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::{Value, json};
use std::marker::PhantomData;

/// Model-facing create goal tool using an application storage host.
/// # Examples
/// ```no_run
/// fn tool<H: codetether_goal_tools::Host>() {
///     let _tool = codetether_goal_tools::CreateGoalTool::<H>::new();
/// }
/// ```
pub struct CreateGoalTool<H>(PhantomData<fn() -> H>);
impl<H> CreateGoalTool<H> {
    /// Construct the tool without allocating or touching storage.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}
impl<H> Default for CreateGoalTool<H> {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<H: Host + 'static> Tool for CreateGoalTool<H> {
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
        if let Some(refused) = codetether_session_task::budget_refusal(&input) {
            return Ok(refused);
        }
        super::create_run::run::<H>(serde_json::from_value(input)?).await
    }
}
