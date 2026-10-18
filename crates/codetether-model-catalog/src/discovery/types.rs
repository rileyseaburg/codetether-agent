//! Response structs for `/models` and `/v1/models`.

pub use super::types_architecture::Architecture;
pub use super::types_pricing::Pricing;
pub use super::types_top_provider::TopProvider;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ModelsResponse {
    pub data: Vec<Model>,
}

#[derive(Debug, Serialize)]
pub struct Model {
    pub id: String,
    pub canonical_slug: String,
    pub name: String,
    pub created: i64,
    pub description: String,
    pub context_length: usize,
    pub architecture: Architecture,
    pub pricing: Pricing,
    pub top_provider: TopProvider,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_request_limits: Option<serde_json::Value>,
    pub supported_parameters: Vec<String>,
    #[serde(flatten)]
    pub options: super::options::ModelOptions,
}
