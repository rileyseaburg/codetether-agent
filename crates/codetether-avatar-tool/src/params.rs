//! Deserialized avatar input and action-specific records.
use serde::Deserialize;
mod records;
pub(crate) use records::{EpisodeAvatarParams, GenerateParams, UploadModelParams};

#[derive(Deserialize)]
pub(crate) struct Params {
    pub action: String,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub audio_url: Option<String>,
    #[serde(default)]
    pub audio_file: Option<String>,
    #[serde(default)]
    pub model_video: Option<String>,
    #[serde(default)]
    pub voice_id: Option<String>,
    #[serde(default)]
    pub podcast_id: Option<String>,
    #[serde(default)]
    pub episode_id: Option<String>,
    #[serde(default)]
    pub upload_youtube: Option<bool>,
    #[serde(default)]
    pub privacy_status: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub file_path: Option<String>,
}
