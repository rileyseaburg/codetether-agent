//! Codex-compatible `followup_task` with same-turn steering.

use super::{context::RuntimeContext, legacy};
use crate::tool::agent::communication::Route;
use crate::tool::{Tool, ToolResult};
use anyhow::{Result, bail};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

pub(super) struct FollowupTaskTool;

#[derive(Deserialize)]
struct Args {
    target: String,
    message: String,
    #[serde(flatten)]
    context: RuntimeContext,
}

#[async_trait]
impl Tool for FollowupTaskTool {
    fn id(&self) -> &str { "followup_task" }
    fn name(&self) -> &str { "Follow Up Task" }
    fn description(&self) -> &str {
        "Steer a running child promptly, or trigger a background turn when it is idle."
    }
    fn parameters(&self) -> Value {
        json!({"type":"object","properties":{
            "target":{"type":"string"}, "message":{"type":"string"}
        },"required":["target","message"]})
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let args: Args = serde_json::from_value(input)?;
        if args.message.trim().is_empty() { bail!("Empty message can't be sent to an agent"); }
        if let Some(result) = super::ensure::ready(&args.context, &args.target).await? {
            return Ok(result);
        }
        match crate::tool::agent::communication::steer(
            &args.target, args.context.session_id.as_deref(), &args.message,
        ).await {
            Route::Steered => Ok(ToolResult::success(String::new())),
            Route::NotFound => Ok(ToolResult::error(format!("Agent {} not found", args.target))),
            Route::Idle => trigger(args).await,
        }
    }
}

async fn trigger(args: Args) -> Result<ToolResult> {
    let mut result = legacy::execute(&args.context, json!({
        "action":"message", "name":args.target,
        "message":args.message, "detach":true
    }).as_object().cloned().expect("object payload")).await?;
    if result.success { result.output.clear(); }
    Ok(result)
}
