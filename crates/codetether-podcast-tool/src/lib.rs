//! Podcast-series and episode management through the configured voice service.
//!
//! [PodcastTool] supports creation, listing, inspection and deletion. Validation,
//! request construction, response formatting and schema each have separate modules.
//!
//! ```rust
//! let _tool = codetether_podcast_tool::PodcastTool::new();
//! ```
use std::time::Duration;
mod create_episode;
mod create_episode_args;
mod create_episode_result;
mod create_podcast;
mod create_podcast_args;
mod create_podcast_result;
mod delete_episode;
mod delete_episode_args;
mod delete_podcast;
mod delete_podcast_args;
mod execute;
mod get_podcast;
mod get_podcast_args;
mod list_podcasts;
mod list_podcasts_result;
mod params;
mod required;
mod schema;
mod schema_fields_0;
mod schema_fields_1;
mod schema_fields_2;
mod schema_fields_3;
mod tool;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(300);
// Episodes can take a while

fn voice_api_url() -> String {
    std::env::var("CODETETHER_VOICE_API_URL")
        .unwrap_or_else(|_| "https://voice.quantum-forge.io".to_string())
}
/// Manage podcast series and generated episodes through the voice API.
///
/// # Examples
/// ```rust
/// let _tool = codetether_podcast_tool::PodcastTool::new();
/// ```
pub struct PodcastTool {
    client: reqwest::Client,
}
impl PodcastTool {
    /// Create a reusable HTTP client with the episode-generation timeout.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// A tool ready for registration.
    /// # Panics
    /// Panics if the HTTP client cannot be constructed.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_podcast_tool::PodcastTool::new();
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

impl Default for PodcastTool {
    fn default() -> Self {
        Self::new()
    }
}
