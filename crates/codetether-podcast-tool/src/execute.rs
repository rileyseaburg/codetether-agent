//! Decode and dispatch podcast actions.
use super::{PodcastTool, params::Params};
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};
impl PodcastTool {
    pub(super) async fn execute_action(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;

        match p.action.as_str() {
            "create_podcast" => self.create_podcast_args(p).await,
            "create_episode" => self.create_episode_args(p).await,
            "list_podcasts" => self.list_podcasts().await,
            "get_podcast" => self.get_podcast_args(p).await,
            "delete_podcast" => self.delete_podcast_args(p).await,
            "delete_episode" => self.delete_episode_args(p).await,
            other => Ok(ToolResult::structured_error(
                "INVALID_ACTION",
                "podcast",
                &format!(
                    "Unknown action '{other}'. Use: create_podcast, create_episode, list_podcasts, get_podcast, delete_podcast, delete_episode"
                ),
                None,
                Some(json!({"action": "list_podcasts"})),
            )),
        }
    }
}
