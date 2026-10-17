//! Model catalog response schema.
use crate::*;
#[derive(Deserialize)]
pub(super) struct ModelsResponse {
    pub(super) data: Vec<ModelData>,
}
#[derive(Deserialize)]
pub(super) struct ModelData {
    pub(super) id: String,
    #[serde(default)]
    pub(super) name: Option<String>,
    #[serde(default)]
    pub(super) context_length: Option<usize>,
    #[serde(default)]
    pub(super) supported_parameters: Vec<String>,
}
