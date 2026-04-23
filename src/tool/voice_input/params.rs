//! Input parameters for the voice_input tool.

use serde::Deserialize;

/// Parameters accepted by the `voice_input` tool.
#[derive(Deserialize)]
pub struct Params {
    /// Action to perform (e.g. "record_then_transcribe").
    pub action: String,
    /// Optional max recording duration in seconds.
    #[serde(default)]
    pub max_duration_secs: Option<u64>,
}
