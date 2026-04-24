//! record_then_transcribe — capture mic audio and POST to Whisper API.

use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use anyhow::Result;

use crate::tool::ToolResult;

use super::super::{encoder, recorder};

/// Maximum allowed recording duration (5 minutes).
const MAX_DURATION: u64 = 300;

/// Default recording duration in seconds.
const DEFAULT_DURATION: u64 = 60;

/// Run the record_then_transcribe action.
///
/// Captures audio from the default mic, encodes to WAV,
/// and sends it to the Voice API `/transcribe` endpoint.
pub(crate) async fn run(
    client: &reqwest::Client,
    max_duration_secs: Option<u64>,
) -> Result<ToolResult> {
    let max_secs = max_duration_secs
        .unwrap_or(DEFAULT_DURATION)
        .min(MAX_DURATION);

    let stop = Arc::new(AtomicBool::new(false));
    let samples = tokio::task::spawn_blocking(move || recorder::record(max_secs, stop))
        .await
        .map_err(|_| anyhow::anyhow!("Recording task panicked"))??;

    if samples.is_empty() {
        return Ok(ToolResult::error(
            "No audio captured. Check your microphone.",
        ));
    }

    let wav_bytes = encoder::encode_wav(&samples)?;
    tracing::info!("WAV encoded: {} bytes", wav_bytes.len());
    super::transcribe_api::post_transcribe(client, &wav_bytes).await
}
