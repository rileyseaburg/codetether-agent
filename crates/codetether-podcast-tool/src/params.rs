//! Deserialized podcast tool input and action records.
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Params {
    pub action: String,
    #[serde(default)]
    pub podcast_id: Option<String>,
    #[serde(default)]
    pub episode_id: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub subcategory: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub script: Option<String>,
    #[serde(default)]
    pub voice_id: Option<String>,
    #[serde(default)]
    pub episode_number: Option<i64>,
    #[serde(default)]
    pub season_number: Option<i64>,
}
mod create_episode;
mod create_podcast;
pub(crate) use create_episode::CreateEpisodeParams;
pub(crate) use create_podcast::CreatePodcastParams;
