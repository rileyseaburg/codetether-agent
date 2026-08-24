//! Codex-compatible `followup_task` with same-turn steering.

#[path = "followup_trigger.rs"]
mod trigger;

use super::context::RuntimeContext;
use crate::tool::agent::communication::Route;
use crate::tool::{Tool, ToolResult};
use anyhow::{Result, bail};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

pub(super) struct FollowupTaskTool;

#[derive(Deserialize)]
pub(super) struct Args {
    pub(super) target: String,
    pub(super) message: String,
    #[serde(flatten)]
    pub(super) context: RuntimeContext,
}

#[async_trait]
impl Tool for FollowupTaskTool {
    fn id(&self) -> &str {
        "followup_task"
    }
    fn name(&self) -> &str {
        "Follow Up Task"
    }
    fn description(&self) -> &str {
        "Steer a running child promptly, or trigger a background turn when it is idle."
    }
    fn parameters(&self) -> Value {
        json!({"type":"object","properties":{
            "target":{"type":"string"}, "message":{"type":"string"}
        },"required":["target","message"]})
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let args: Args = serde_json::from_value(input.clone())?;
        if args.message.trim().is_empty() {
            bail!("Empty message can't be sent to an agent");
        }
        let authority = match super::authority::claim("followup_task", &input).await {
            Ok(authority) => authority,
            Err(blocked) => return Ok(blocked),
        };
        if let Some(result) = super::ensure::ready(&args.context, &args.target).await? {
            return Ok(result);
        }
        match crate::tool::agent::communication::steer(
            &args.target,
            args.context.session_id.as_deref(),
            &args.message,
        )
        .await
        {
            Route::Steered => Ok(ToolResult::success(String::new())),
            Route::NotFound => Ok(ToolResult::error(format!("Agent {} not found", args.target))),
            Route::Idle => trigger::run(args, authority).await,
        }
    }
}