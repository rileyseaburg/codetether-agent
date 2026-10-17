//! Action-specific avatar request records.
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct GenerateParams {
    pub text: Option<String>,
    pub audio_url: Option<String>,
    pub audio_file: Option<String>,
    pub model_video: Option<String>,
    pub voice_id: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct EpisodeAvatarParams {
    pub podcast_id: String,
    pub episode_id: String,
    pub model_video: Option<String>,
    pub upload_youtube: bool,
    pub privacy_status: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct UploadModelParams {
    pub name: String,
    pub file_path: String,
}
