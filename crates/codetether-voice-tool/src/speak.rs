//! Build and submit a speech-generation request.
use super::{VoiceTool, default_voice_api_url, params::SpeakParams};
use anyhow::Result;
use codetether_tool_core::ToolResult;
impl VoiceTool {
    pub(super) async fn speak(&self, params: &SpeakParams) -> Result<ToolResult> {
        let base_url = default_voice_api_url();
        let voice_id = params.voice_id.as_deref().unwrap_or("960f89fc");

        let url = format!("{base_url}/voices/{voice_id}/speak");

        let lang = params.language.clone().unwrap_or_else(|| "english".into());

        let form = reqwest::multipart::Form::new()
            .text("text", params.text.clone())
            .text("language", lang);

        let resp = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Voice API request failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "Voice API returned {status}: {body}"
            )));
        }

        super::speak_result::save(resp).await
    }
}
