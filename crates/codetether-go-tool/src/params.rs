//! Decoded action arguments and optional pipeline limits.

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct GoParams {
    pub(crate) action: String,
    #[serde(default)]
    pub(crate) task: Option<String>,
    #[serde(default)]
    pub(crate) max_iterations: Option<usize>,
    #[serde(default)]
    pub(crate) max_concurrent_stories: Option<usize>,
    #[serde(default)]
    pub(crate) model: Option<String>,
    #[serde(default)]
    pub(crate) okr_id: Option<String>,
}
