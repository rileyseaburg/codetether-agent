//! Convert internal provider model metadata to API models.

use super::{ids, types};

type ModelInfo = crate::provider::ModelInfo;

pub(crate) fn convert_model(provider: &str, model: ModelInfo) -> types::Model {
    let id = ids::model_id(provider, &model.id);
    types::Model {
        id: id.clone(),
        canonical_slug: id,
        name: ids::display_name(provider, &model.name),
        created: 0,
        description: format!(
            "{} model served by CodeTether provider {provider}",
            model.name
        ),
        context_length: model.context_window,
        architecture: architecture(&model),
        pricing: super::pricing::price(&model),
        top_provider: top_provider(&model),
        per_request_limits: None,
        supported_parameters: super::parameters::supported_parameters(&model),
    }
}

fn architecture(model: &ModelInfo) -> types::Architecture {
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

fn top_provider(model: &ModelInfo) -> types::TopProvider {
    types::TopProvider {
        context_length: model.context_window,
        max_completion_tokens: model.max_output_tokens,
        is_moderated: false,
    }
}
