//! Deserialized input for speech generation and playback.
use serde::Deserialize;

/// Input parameters deserialized from the LLM call.
///
/// # Examples
/// ```rust
/// use codetether_voice_stream_tool::Params;
/// let input = Params {
///     action: "play".into(), job_id: Some("job-123".into()),
///     text: None, voice_id: None, language: None,
/// };
/// assert_eq!(input.action, "play");
/// ```
#[derive(Deserialize)]
pub struct Params {
    pub action: String,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub voice_id: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub job_id: Option<String>,
}
