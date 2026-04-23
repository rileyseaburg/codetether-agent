//! Voice API URL resolution for voice_input tool.

/// Resolve the Voice API base URL from environment.
pub(crate) fn api_url() -> String {
    std::env::var("CODETETHER_VOICE_API_URL")
        .unwrap_or_else(|_| "https://voice.quantum-forge.io".to_string())
}

/// Build the HTTP client used for transcription requests.
pub(crate) fn build_client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .user_agent("CodeTether-Agent/1.0")
        .build()
        .expect("Failed to build HTTP client")
}
