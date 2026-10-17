//! Parameters for generating an episode.
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct CreateEpisodeParams {
    pub podcast_id: String,
    pub title: String,
    pub script: String,
    pub voice_id: Option<String>,
    pub description: Option<String>,
    pub episode_number: Option<i64>,
    pub season_number: Option<i64>,
}
