//! Voice-service speech streaming and browser playback.
//!
//! [VoiceStreamTool] generates speech or replays an existing job. Request,
//! playback, parameter and schema modules keep these responsibilities separate.
//!
//! ```rust
//! let _tool = codetether_voice_stream_tool::VoiceStreamTool::new();
//! ```
mod actions;
mod client;
mod params;
mod schema;
mod tool;
use codetether_tool_core::ToolResult;
pub use params::Params;
/// Generate speech and open an HTML audio player in the system browser.
///
/// # Examples
/// ```rust
/// let _tool = codetether_voice_stream_tool::VoiceStreamTool::new();
/// ```

pub struct VoiceStreamTool {
    client: reqwest::Client,
}

/// Resolve the Voice API base URL from env.
pub(crate) fn voice_api_url() -> String {
    std::env::var("CODETETHER_VOICE_API_URL")
        .unwrap_or_else(|_| "https://voice.quantum-forge.io".to_string())
}
