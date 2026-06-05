//! Architecture modality structs.

use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct Architecture {
    pub(crate) modality: String,
    pub(crate) input_modalities: Vec<String>,
    pub(crate) output_modalities: Vec<String>,
    pub(crate) tokenizer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) instruct_type: Option<String>,
}
