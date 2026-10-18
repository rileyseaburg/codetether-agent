//! Codex message record payload schemas.
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct CodexResponseMessagePayload {
    pub(crate) role: String,
    #[serde(default)]
    pub(crate) content: Vec<CodexContentItem>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CodexContentItem {
    #[serde(rename = "type")]
    pub(crate) kind: String,
    #[serde(default)]
    pub(crate) text: Option<String>,
    #[serde(default)]
    pub(crate) image_url: Option<String>,
}
