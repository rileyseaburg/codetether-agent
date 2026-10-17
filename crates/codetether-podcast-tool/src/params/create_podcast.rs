//! Parameters for creating a podcast series.
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct CreatePodcastParams {
    pub title: String,
    pub description: String,
    pub author: Option<String>,
    pub email: Option<String>,
    pub category: Option<String>,
    pub subcategory: Option<String>,
    pub language: Option<String>,
}
