//! Decode and dispatch publishing actions.
use super::{YouTubeTool, params::Params};
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};
impl YouTubeTool {
    pub(super) async fn execute_action(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;

        match p.action.as_str() {
            "publish_episode" => self.publish_args(p).await,
            "upload_video" => self.upload_args(p).await,
            "status" => self.status().await,
            other => Ok(ToolResult::structured_error(
                "INVALID_ACTION",
                "youtube",
                &format!("Unknown action '{other}'. Use: publish_episode, upload_video, status"),
                None,
                Some(json!({"action": "status"})),
            )),
        }
    }
}
