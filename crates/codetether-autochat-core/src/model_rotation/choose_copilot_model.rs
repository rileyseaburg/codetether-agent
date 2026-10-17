//! Auto-chat choose copilot model.
use super::*;

pub(super) fn choose_copilot_model(models: &[ModelInfo]) -> Option<String> {
    let preferred = ["gpt-5-mini", "gpt-4.1", "gpt-4o"];
    for model_id in preferred {
        if let Some(found) = models
            .iter()
            .find(|model| model.id.eq_ignore_ascii_case(model_id))
        {
            return Some(found.id.clone());
        }
    }

    models
        .iter()
        .find(|model| {
            model.input_cost_per_million == Some(0.0) && model.output_cost_per_million == Some(0.0)
        })
        .map(|model| model.id.clone())
}
