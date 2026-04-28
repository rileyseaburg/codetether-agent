//! TTS request builder for voice stream.

use super::super::Params;
use anyhow::Result;
use serde_json::json;

pub(super) async fn send(
    client: &reqwest::Client,
    params: &Params,
    base: &str,
    text: &str,
) -> Result<reqwest::Response> {
    let body = json!({
        "script": text,
        "voice_id": params.voice_id.as_deref().unwrap_or("960f89fc"),
        "language": params.language.as_deref().unwrap_or("english")
    });
    client
        .post(format!("{base}/tts/speak"))
        .json(&body)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("TTS request failed: {e}"))
}
