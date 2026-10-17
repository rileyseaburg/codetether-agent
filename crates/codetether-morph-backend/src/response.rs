//! Deserialized direct OpenRouter completion records.
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct MorphResponse {
    pub(super) choices: Vec<MorphChoice>,
}

#[derive(Debug, Deserialize)]
pub(super) struct MorphChoice {
    pub(super) message: MorphMessage,
}

#[derive(Debug, Deserialize)]
pub(super) struct MorphMessage {
    #[serde(default)]
    pub(super) content: Option<String>,
}
