//! Publishing and upload parameters with their existing defaults.
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Params {
    pub action: String,
    #[serde(default)]
    pub podcast_id: Option<String>,
    #[serde(default)]
    pub episode_id: Option<String>,
    #[serde(default)]
    pub privacy_status: Option<String>,
    #[serde(default)]
    pub custom_title: Option<String>,
    #[serde(default)]
    pub custom_description: Option<String>,
    #[serde(default)]
    pub tags: Option<String>,
    #[serde(default)]
    pub file_path: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub category_id: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct PublishEpisodeParams {
    pub podcast_id: String,
    pub episode_id: String,
    pub privacy_status: Option<String>,
    pub custom_title: Option<String>,
    pub custom_description: Option<String>,
    pub tags: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct UploadVideoParams {
    pub file_path: String,
    pub title: String,
    pub description: Option<String>,
    pub tags: Option<String>,
    pub privacy_status: Option<String>,
    pub category_id: Option<String>,
}
