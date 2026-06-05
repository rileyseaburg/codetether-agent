//! Response structs for `/models` and `/v1/models`.

use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct ModelsResponse {
    pub(crate) data: Vec<Model>,
}

#[derive(Debug, Serialize)]
pub(crate) struct Model {
    pub(crate) id: String,
    pub(crate) canonical_slug: String,
    pub(crate) name: String,
    pub(crate) created: i64,
    pub(crate) description: String,
    pub(crate) context_length: usize,
    pub(crate) architecture: Architecture,
    pub(crate) pricing: Pricing,
    pub(crate) top_provider: TopProvider,
    pub(crate) per_request_limits: Option<serde_json::Value>,
    pub(crate) supported_parameters: Vec<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct Architecture {
    pub(crate) modality: String,
    pub(crate) input_modalities: Vec<String>,
    pub(crate) output_modalities: Vec<String>,
    pub(crate) tokenizer: String,
    pub(crate) instruct_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct Pricing {
    pub(crate) prompt: String,
    pub(crate) completion: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct TopProvider {
    pub(crate) context_length: usize,
    pub(crate) max_completion_tokens: Option<usize>,
    pub(crate) is_moderated: bool,
}
