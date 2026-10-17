//! Configure and construct the reusable voice-service client.
use super::VoiceStreamTool;

impl Default for VoiceStreamTool {
    fn default() -> Self {
        Self::new()
    }
}
impl VoiceStreamTool {
    /// Construct a client with a two-minute request timeout.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// A tool ready for registration.
    /// # Panics
    /// Panics if the HTTP client cannot be constructed.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_voice_stream_tool::VoiceStreamTool::new();
    /// ```

    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .user_agent("CodeTether-Agent/1.0")
                .build()
                .expect("Failed to build HTTP client"),
        }
    }
}
