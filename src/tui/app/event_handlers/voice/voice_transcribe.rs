//! Synchronous Voice API transcription (blocking HTTP POST).

/// POST WAV bytes to the Voice API and return the transcribed text.
pub(crate) fn transcribe_sync(wav: &[u8]) -> Option<String> {
    let url = format!(
        "{}/transcribe",
        std::env::var("CODETETHER_VOICE_API_URL")
            .unwrap_or_else(|_| "https://voice.quantum-forge.io".into())
    );
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .ok()?;
    let part = reqwest::blocking::multipart::Part::bytes(wav.to_vec())
        .file_name("recording.wav")
        .mime_str("audio/wav")
        .ok()?;
    let form = reqwest::blocking::multipart::Form::new().part("audio_file", part);
    let resp = client.post(&url).multipart(form).send().ok()?;
    if !resp.status().is_success() {
        tracing::error!("Voice API error: {}", resp.status());
        return None;
    }
    let body: serde_json::Value = resp.json().ok()?;
    Some(body["transcription"].as_str()?.to_string())
}
