//! `get_goal` tool implementation.

use super::{context, response};
use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

pub(super) struct GetGoalTool;

#[derive(Deserialize)]
struct Args {
    #[serde(default, rename = "__ct_session_id")]
    session_id: Option<String>,
}

#[async_trait]
impl Tool for GetGoalTool {
    fn id(&self) -> &str {
        "get_goal"
    }
    fn name(&self) -> &str {
        "Get Goal"
    }
    fn description(&self) -> &str {
        "Get the current goal for this session, including status, budgets, token and elapsed-time usage, and remaining token budget."
    }
    fn parameters(&self) -> Value {
        json!({"type":"object","properties":{}})
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let args: Args = serde_json::from_value(input)?;
        let session_id = context::session_id(args.session_id)?;
        let (_, state) = crate::session::tasks::runtime::current(&session_id).await?;
        Ok(response::result(&state))
    }
}
