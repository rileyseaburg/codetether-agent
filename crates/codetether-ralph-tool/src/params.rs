//! Decoded Ralph action arguments.

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Params {
    pub(crate) action: String,
    #[serde(default)]
    pub(crate) prd_path: Option<String>,
    #[serde(default)]
    pub(crate) feature: Option<String>,
    #[serde(default)]
    pub(crate) project: Option<String>,
    #[serde(default)]
    pub(crate) max_iterations: Option<usize>,
}
