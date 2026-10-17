//! YouTube publishing through the configured voice-service integration.
//!
//! [YouTubeTool] publishes podcast episodes, uploads video files and reports
//! channel status. Separate modules own validation, requests and responses.
//!
//! ```rust
//! let _tool = codetether_youtube_tool::YouTubeTool::new();
//! ```
use std::time::Duration;
mod execute;
mod params;
mod publish;
mod publish_args;
mod publish_fields;
mod publish_result;
mod schema;
mod schema_fields;
mod status;
mod status_result;
mod tool;
mod upload;
mod upload_args;
mod upload_fields;
mod upload_form;
mod upload_result;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(600);
// Video generation + upload can be slow

fn voice_api_url() -> String {
    std::env::var("CODETETHER_VOICE_API_URL")
        .unwrap_or_else(|_| "https://voice.quantum-forge.io".to_string())
}
/// Publish videos and query channel status through the voice API.
///
/// # Examples
/// ```rust
/// let _tool = codetether_youtube_tool::YouTubeTool::new();
/// ```
pub struct YouTubeTool {
    client: reqwest::Client,
}
impl YouTubeTool {
    /// Create a reusable client with the existing upload timeout.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// A tool ready for registration.
    /// # Panics
    /// Panics if the HTTP client cannot be constructed.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_youtube_tool::YouTubeTool::new();
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

impl Default for YouTubeTool {
    fn default() -> Self {
        Self::new()
    }
}
