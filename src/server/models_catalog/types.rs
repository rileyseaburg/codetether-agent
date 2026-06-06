//! Response structs for `/models` and `/v1/models`.

pub(crate) use super::types_architecture::Architecture;
pub(crate) use super::types_pricing::Pricing;
pub(crate) use super::types_top_provider::TopProvider;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) per_request_limits: Option<serde_json::Value>,
    pub(crate) supported_parameters: Vec<String>,
}
