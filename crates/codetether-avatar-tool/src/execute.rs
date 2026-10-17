//! Decode and dispatch avatar actions.
use super::{AvatarTool, params::Params};
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};
impl AvatarTool {
    pub(super) async fn execute_action(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;

        match p.action.as_str() {
            "status" => self.status().await,
            "start" => self.start().await,
            "stop" => self.stop().await,
            "generate" => self.generate_args(p).await,
            "generate_from_episode" => self.generate_from_episode_args(p).await,
            "list_models" => self.list_models().await,
            "upload_model" => self.upload_model_args(p).await,
            other => Ok(ToolResult::structured_error(
                "INVALID_ACTION",
                "avatar",
                &format!(
                    "Unknown action '{other}'. Use: status, start, stop, generate, generate_from_episode, list_models, upload_model"
                ),
                None,
                Some(json!({"action": "status"})),
            )),
        }
    }
}
