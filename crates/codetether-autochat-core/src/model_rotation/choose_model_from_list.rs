//! Auto-chat choose model from list.
use super::*;

pub(super) fn choose_model_from_list(provider_name: &str, models: &[ModelInfo]) -> Option<String> {
    if models.is_empty() {
        return None;
    }

    if provider_name == "openrouter" {
        return choose_openrouter_model(models).or_else(|| models.first().map(|m| m.id.clone()));
    }

    if provider_name == "github-copilot" || provider_name == "github-copilot-enterprise" {
        return choose_copilot_model(models).or_else(|| models.first().map(|m| m.id.clone()));
    }

    let preferred = preferred_models_for_provider(provider_name);
    for model_id in preferred {
        if let Some(found) = models
            .iter()
            .find(|model| model.id.eq_ignore_ascii_case(model_id))
        {
            return Some(found.id.clone());
        }
    }

    models.first().map(|m| m.id.clone())
}
