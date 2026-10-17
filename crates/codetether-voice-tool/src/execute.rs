//! Decode and dispatch voice-service actions.
use super::{VoiceTool, params::Params};
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};
impl VoiceTool {
    pub(super) async fn execute_action(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;

        match p.action.as_str() {
            "speak" => self.speak_args(p).await,
            "transcribe" => self.transcribe_args(p).await,
            "list_voices" => self.list_voices().await,
            "health" => self.health().await,
            other => Ok(ToolResult::structured_error(
                "INVALID_ACTION",
                "voice",
                &format!("Unknown action '{other}'. Use: speak, transcribe, list_voices, health"),
                None,
                Some(json!({"action": "speak", "text": "Hello world"})),
            )),
        }
    }
}
