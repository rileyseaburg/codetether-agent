//! Voice-service speech generation, transcription and profile discovery.
//!
//! [VoiceTool] builds requests for the configured voice API. Speech output is
//! saved in the current directory and returned with job and byte metadata.
//!
//! ```rust
//! let _tool = codetether_voice_tool::VoiceTool::new();
//! ```
use std::time::Duration;
mod execute;
mod health;
mod params;
mod schema;
mod speak;
mod speak_args;
mod speak_result;
mod tool;
mod transcribe;
mod transcribe_args;
mod transcribe_result;
mod voices;
mod voices_result;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(120);

fn default_voice_api_url() -> String {
    std::env::var("CODETETHER_VOICE_API_URL")
        .unwrap_or_else(|_| "https://voice.quantum-forge.io".to_string())
}
/// Remote voice-service tool with a reusable HTTP client.
///
/// # Examples
/// ```rust
/// let _tool = codetether_voice_tool::VoiceTool::new();
/// ```
pub struct VoiceTool {
    client: reqwest::Client,
}
impl VoiceTool {
    /// Create the HTTP client with the existing voice-service timeout.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// A voice tool with a reusable client.
    /// # Panics
    /// Panics if the HTTP client cannot be constructed.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_voice_tool::VoiceTool::new();
    /// ```

    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .user_agent("CodeTether-Agent/1.0")
            .build()
            .expect("Failed to build HTTP client");
        Self { client }
    }
}

impl Default for VoiceTool {
    fn default() -> Self {
        Self::new()
    }
}
