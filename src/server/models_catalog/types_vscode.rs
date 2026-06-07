//! VS Code language-model shaped model discovery response.

use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VscodeModelsResponse {
    pub(crate) data: Vec<VscodeModel>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VscodeModel {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) vendor: String,
    pub(crate) family: String,
    pub(crate) max_input_tokens: usize,
    pub(crate) max_output_tokens: Option<usize>,
    pub(crate) supports_tools: bool,
    pub(crate) supports_vision: bool,
}
