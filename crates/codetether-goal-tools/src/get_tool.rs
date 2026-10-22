//! `get_goal` tool implementation.

use super::{context, response};
use crate::Host;
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde::Deserialize;
use serde_json::{Value, json};
use std::marker::PhantomData;

/// Model-facing get goal tool using an application storage host.
/// # Examples
/// ```no_run
/// fn tool<H: codetether_goal_tools::Host>() {
///     let _tool = codetether_goal_tools::GetGoalTool::<H>::new();
/// }
/// ```
pub struct GetGoalTool<H>(PhantomData<fn() -> H>);
impl<H> GetGoalTool<H> {
    /// Construct the tool without allocating or touching storage.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}
impl<H> Default for GetGoalTool<H> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Deserialize)]
struct Args {
    #[serde(default, rename = "__ct_session_id")]
    session_id: Option<String>,
}

#[async_trait]
impl<H: Host + 'static> Tool for GetGoalTool<H> {
    fn id(&self) -> &str {
        "get_goal"
    }
    fn name(&self) -> &str {
        "Get Goal"
    }
    fn description(&self) -> &str {
        "Get the current goal for this session, including status, budgets, token and elapsed-time usage, remaining token budget, and recorded user-message provenance."
    }
    fn parameters(&self) -> Value {
        json!({"type":"object","properties":{}})
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let args: Args = serde_json::from_value(input)?;
        let session_id = context::session_id(args.session_id)?;
        let (log, state) = H::current(&session_id).await?;
        Ok(response::with_source(&state, &log.read_all().await?))
    }
}
