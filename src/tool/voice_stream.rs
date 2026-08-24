//! Voice Stream Tool - Stream and play TTS audio via browser.
//!
//! Provides `speak_stream` (text → TTS → open in browser) and `play`
//! (play a previously generated job_id in the browser).

mod actions;
mod schema;

use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;

pub struct VoiceStreamTool {
    client: reqwest::Client,
}

impl Default for VoiceStreamTool {
    fn default() -> Self {
        Self::new()
    }
}

impl VoiceStreamTool {
    pub fn new() -> Self {
        Self {
            client: super::network_access::no_redirect_client_with_timeout(
                std::time::Duration::from_secs(120),
            )
            .expect("voice stream HTTP client"),
        }
    }
}

/// Resolve the Voice API base URL from env.
pub(crate) fn voice_api_url() -> String {
    std::env::var("CODETETHER_VOICE_API_URL")
        .unwrap_or_else(|_| "https://voice.quantum-forge.io".to_string())
}

/// Input parameters deserialized from the LLM call.
#[derive(Deserialize)]
pub struct Params {
    pub action: String,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub voice_id: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub job_id: Option<String>,
}

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
        let p: Params = crate::tool::network_access::args!("voice_stream", params);
        actions::dispatch(&self.client, &p).await
    }
}
