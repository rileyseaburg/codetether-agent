//! Webfetch parameter decoding.

use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Params {
    pub url: String,
    #[serde(default = "default_fmt")]
    pub format: String,
    #[serde(default = "default_max_chars")]
    pub max_chars: usize,
    #[serde(default, rename = "__ct_parent_workspace")]
    pub parent_workspace: Option<String>,
}

fn default_fmt() -> String {
    "markdown".into()
}

fn default_max_chars() -> usize {
    super::DEFAULT_MAX_CHARS
}
