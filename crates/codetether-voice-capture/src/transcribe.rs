//! Synchronous Voice API transcription (blocking HTTP POST).

/// POST WAV bytes to the Voice API and return the transcribed text.
/// # Arguments
/// * `wav` — Encoded audio sent as `recording.wav` in the multipart request.
/// # Returns
/// The transcription text.
/// # Errors
/// Returns a user-facing reason on transport, HTTP status, response-format
/// errors, or when the service heard no speech.
/// # Examples
/// ```no_run
/// let text = codetether_voice_capture::transcribe_sync(&[]);
/// # let _ = text;
/// ```
pub fn transcribe_sync(wav: &[u8]) -> Result<String, String> {
    let base = std::env::var("CODETETHER_VOICE_API_URL")
        .unwrap_or_else(|_| "https://voice.quantum-forge.io".into());
    let url = format!("{base}/transcribe");
    let fail = |what: &str, e: &dyn std::fmt::Display| {
        tracing::error!(target: "codetether_agent::tui::app::event_handlers::voice::voice_transcribe", url = %url, error = %e, "{what}");
        format!("{what}: {e} (CODETETHER_VOICE_API_URL={base})")
    };
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|e| fail("Voice HTTP client failed", &e))?;
    let part = reqwest::blocking::multipart::Part::bytes(wav.to_vec())
        .file_name("recording.wav")
        .mime_str("audio/wav")
        .map_err(|e| fail("Voice request build failed", &e))?;
    let form = reqwest::blocking::multipart::Form::new().part("audio_file", part);
    let resp = client
        .post(&url)
        .multipart(form)
        .send()
        .map_err(|e| fail("Voice API unreachable", &e))?;
    let status = resp.status();
    if !status.is_success() {
        return Err(fail("Voice API error", &status));
    }
    let body: serde_json::Value = resp
        .json()
        .map_err(|e| fail("Voice API returned invalid JSON", &e))?;
    let text = body["transcription"].as_str().unwrap_or_default().trim();
    if text.is_empty() {
        return Err(
            "Voice API heard no speech — speak closer to the mic or check input level".into(),
        );
    }
    Ok(text.to_string())
}
