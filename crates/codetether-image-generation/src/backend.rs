//! Explicit image backend selection; existing calls retain OpenAI behavior.
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub(super) enum ImageBackend {
    #[default]
    #[serde(alias = "openai-codex")]
    Openai,
    GeminiWeb,
}
