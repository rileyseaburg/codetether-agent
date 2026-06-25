use super::{actions, client, params, schema};
use crate::tool::{Tool, ToolResult};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde_json::Value;

/// Voice Input tool — records from mic and transcribes to text.
pub struct VoiceInputTool {
    client: reqwest::Client,
}

impl Default for VoiceInputTool {
    fn default() -> Self {
        Self::new()
    }
}

impl VoiceInputTool {
    /// Create a new voice input tool with a configured HTTP client.
    pub fn new() -> Self {
        Self {
            client: client::build_client(),
        }
    }
}

#[async_trait]
impl Tool for VoiceInputTool {
    fn id(&self) -> &str {
        "voice_input"
    }
    fn name(&self) -> &str {
        "VoiceInput"
    }
    fn description(&self) -> &str {
        "Record audio from microphone and transcribe to text. Action: record_then_transcribe."
    }
    fn parameters(&self) -> Value {
        schema::json_schema()
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: params::Params =
            serde_json::from_value(params).context("Invalid voice_input params")?;
        actions::dispatch(&self.client, &p).await
    }
}
