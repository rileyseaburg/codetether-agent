//! Query voice-service health with the existing short timeout.
use super::{VoiceTool, default_voice_api_url};
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::Value;
use std::time::Duration;
impl VoiceTool {
    pub(super) async fn health(&self) -> Result<ToolResult> {
        let base_url = default_voice_api_url();
        let url = format!("{base_url}/health");

        let resp = self
            .client
            .get(&url)
            .timeout(Duration::from_secs(5))
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Voice API health check failed: {e}"))?;

        let body: Value = resp
            .json()
            .await
            .context("Failed to parse health response")?;
        let status = body["status"].as_str().unwrap_or("unknown");
        let tts_loaded = body["tts_model_loaded"].as_bool().unwrap_or(false);
        let whisper_loaded = body["whisper_model_loaded"].as_bool().unwrap_or(false);

        Ok(ToolResult::success(format!(
            "Voice API Status: {status}\nTTS model: {}\nWhisper model: {}",
            if tts_loaded { "loaded" } else { "not loaded" },
            if whisper_loaded {
                "loaded"
            } else {
                "not loaded"
            },
        )))
    }
}
