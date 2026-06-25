//! record_then_transcribe — capture mic audio and POST to Whisper API.

use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use anyhow::Result;

use crate::tool::ToolResult;

use super::super::{encoder, recorder};

const MAX_DURATION: u64 = 300;
const DEFAULT_DURATION: u64 = 60;

/// Run the record_then_transcribe action.
pub(crate) async fn run(
    client: &reqwest::Client,
    max_duration_secs: Option<u64>,
    device: Option<&str>,
) -> Result<ToolResult> {
    let max_secs = max_duration_secs
        .unwrap_or(DEFAULT_DURATION)
        .min(MAX_DURATION);
    let selected = device.map(str::to_owned);
    let stop = Arc::new(AtomicBool::new(false));
    let samples = tokio::task::spawn_blocking(move || {
        recorder::record_from(selected.as_deref(), max_secs, stop)
    })
    .await
    .map_err(|_| anyhow::anyhow!("Recording task panicked"))??;
    transcribe_samples(client, &samples).await
}

async fn transcribe_samples(client: &reqwest::Client, samples: &[i16]) -> Result<ToolResult> {
    if samples.is_empty() {
        return Ok(ToolResult::error(
            "No audio captured. Check your microphone.",
        ));
    }
    let wav_bytes = encoder::encode_wav(samples)?;
    tracing::info!(bytes = wav_bytes.len(), "WAV encoded");
    super::transcribe_api::post_transcribe(client, &wav_bytes).await
}
