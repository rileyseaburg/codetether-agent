//! Architecture modality structs.

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Architecture {
    pub modality: String,
    pub input_modalities: Vec<String>,
    pub output_modalities: Vec<String>,
    pub tokenizer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruct_type: Option<String>,
}
