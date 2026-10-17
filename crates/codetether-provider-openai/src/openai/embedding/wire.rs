//! wire for the OpenAI-compatible transport.

#[derive(Debug, serde::Serialize)]
pub(super) struct OpenAIEmbeddingRequest {
    pub(super) model: String,
    pub(super) input: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct OpenAIEmbeddingResponse {
    pub(super) data: Vec<OpenAIEmbeddingData>,
    #[serde(default)]
    pub(super) usage: OpenAIEmbeddingUsage,
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct OpenAIEmbeddingData {
    pub(super) index: usize,
    pub(super) embedding: Vec<f32>,
}

#[derive(Debug, Default, serde::Deserialize)]
pub(super) struct OpenAIEmbeddingUsage {
    #[serde(default)]
    pub(super) prompt_tokens: Option<u32>,
    #[serde(default)]
    pub(super) total_tokens: Option<u32>,
}
