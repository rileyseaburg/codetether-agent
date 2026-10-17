//! Voice-input tool registration and action dispatch.
use super::{actions, client, params, schema};
use anyhow::{Context, Result};
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;

/// Record microphone audio and transcribe it through the configured voice API.
///
/// # Examples
/// ```rust
/// let _tool = codetether_voice_input_tool::VoiceInputTool::new();
/// ```
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
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// A tool ready for registration.
    /// # Panics
    /// Panics if the HTTP client cannot be constructed.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_voice_input_tool::VoiceInputTool::new();
    /// ```
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
