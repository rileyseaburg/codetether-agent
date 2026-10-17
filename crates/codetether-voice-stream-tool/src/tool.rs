//! Tool metadata and execution entry point.
use super::{Params, VoiceStreamTool, actions, schema};
use anyhow::{Context, Result};
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;

#[async_trait]
impl Tool for VoiceStreamTool {
    fn id(&self) -> &str {
        "voice_stream"
    }
    fn name(&self) -> &str {
        "VoiceStream"
    }
    fn description(&self) -> &str {
        "Stream/play TTS audio in the browser. Actions: speak_stream, play."
    }
    fn parameters(&self) -> Value {
        schema::json_schema()
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid voice_stream params")?;
        actions::dispatch(&self.client, &p).await
    }
}
