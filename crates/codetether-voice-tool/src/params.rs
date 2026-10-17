//! Deserialized voice actions and action-specific request records.
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Params {
    pub action: String,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub voice_id: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub file_path: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct SpeakParams {
    pub text: String,
    pub voice_id: Option<String>,
    pub language: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct TranscribeParams {
    pub file_path: String,
}
