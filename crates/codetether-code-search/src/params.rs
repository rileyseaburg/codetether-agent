//! params for workspace code search.
use super::*;

#[derive(Deserialize)]
pub(crate) struct Params {
    pub(crate) pattern: String,
    #[serde(default)]
    pub(crate) path: Option<String>,
    #[serde(default)]
    pub(crate) file_pattern: Option<String>,
    #[serde(default = "default_context")]
    pub(crate) context_lines: usize,
    #[serde(default)]
    pub(crate) case_sensitive: bool,
}

fn default_context() -> usize {
    2
}
