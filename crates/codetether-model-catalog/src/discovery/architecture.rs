//! Architecture and modality projection.
use super::types;
use codetether_provider_core::ModelInfo;

pub(super) fn architecture(model: &ModelInfo) -> types::Architecture {
    let inputs = if model.supports_vision {
        vec!["text", "image"]
    } else {
        vec!["text"]
    };
    types::Architecture {
        modality: format!("{}->text", inputs.join("+")),
        input_modalities: inputs.into_iter().map(str::to_string).collect(),
        output_modalities: vec!["text".to_string()],
        tokenizer: "Other".to_string(),
        instruct_type: None,
    }
}
