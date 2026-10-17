//! params for web search.
use super::*;

#[derive(Deserialize)]
pub(crate) struct Params {
    pub(crate) query: String,
    #[serde(default = "default_max")]
    pub(crate) max_results: usize,
}

fn default_max() -> usize {
    5
}
