//! Adapter from CodeTether model catalog entries to VS Code LM-style records.

use super::{types::Model, types_vscode};

pub(crate) fn requested(format: Option<&str>) -> bool {
    matches!(format, Some("vscode" | "vscode-lm" | "language-model"))
}

pub(crate) fn response(models: &[Model]) -> types_vscode::VscodeModelsResponse {
    types_vscode::VscodeModelsResponse {
        data: models.iter().map(convert).collect(),
    }
}

fn convert(model: &Model) -> types_vscode::VscodeModel {
    let (vendor, family) = split_id(&model.id);
    types_vscode::VscodeModel {
        id: format!("{vendor}/{family}"),
        name: model.name.clone(),
        vendor,
        family,
        max_input_tokens: model.context_length,
        max_output_tokens: model.top_provider.max_completion_tokens,
        supports_tools: model.supported_parameters.iter().any(|p| p == "tools"),
        supports_vision: model
            .architecture
            .input_modalities
            .iter()
            .any(|m| m == "image"),
    }
}

fn split_id(id: &str) -> (String, String) {
    id.split_once('/')
        .map(|(vendor, family)| (vendor.to_string(), family.to_string()))
        .unwrap_or_else(|| ("codetether".to_string(), id.to_string()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn requested_accepts_vscode_aliases() {
        assert!(super::requested(Some("vscode")));
        assert!(super::requested(Some("vscode-lm")));
        assert!(!super::requested(None));
    }
}
