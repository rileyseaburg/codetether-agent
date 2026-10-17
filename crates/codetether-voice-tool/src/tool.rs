//! Tool metadata and execution entry point.
use super::VoiceTool;
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
#[async_trait]
impl Tool for VoiceTool {
    fn id(&self) -> &str {
        "voice"
    }

    fn name(&self) -> &str {
        "Voice"
    }

    fn description(&self) -> &str {
        "Text-to-speech, transcription, and voice management via Qwen TTS. Actions: speak (text to speech with cloned voice), transcribe (audio file to text), list_voices (show available voice profiles), health (check API status). Set CODETETHER_VOICE_API_URL to override the default endpoint."
    }
    fn parameters(&self) -> Value {
        super::schema::parameters()
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        self.execute_action(params).await
    }
}
