//! Auto-chat choose openrouter model.
use super::*;

pub(super) fn choose_openrouter_model(models: &[ModelInfo]) -> Option<String> {
    let free_models: Vec<&ModelInfo> = models
        .iter()
        .filter(|model| is_openrouter_free_model(&model.id))
        .collect();
    if !free_models.is_empty() {
        return free_models
            .into_iter()
            .max_by_key(|model| score_openrouter_model(&model.id))
            .map(|model| model.id.clone());
    }
    models
        .iter()
        .max_by_key(|model| score_openrouter_model(&model.id))
        .map(|model| model.id.clone())
}
