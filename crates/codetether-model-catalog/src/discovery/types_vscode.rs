//! VS Code language-model shaped model discovery response.

use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VscodeModelsResponse {
    pub data: Vec<VscodeModel>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VscodeModel {
    pub id: String,
    pub name: String,
    pub vendor: String,
    pub family: String,
    pub max_input_tokens: usize,
    pub max_output_tokens: Option<usize>,
    pub supports_tools: bool,
    pub supports_vision: bool,
}
