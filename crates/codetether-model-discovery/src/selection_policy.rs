//! Product policy for models exposed by the interactive selector.

use codetether_provider_core::ModelInfo;

pub(super) fn model_ref(provider: &str, model: &ModelInfo) -> Option<String> {
    let provider = provider.trim();
    let model_id = model.id.trim();
    if provider.is_empty() || model_id.is_empty() {
        return None;
    }
    let prefix = format!("{provider}/");
    Some(if model_id.starts_with(&prefix) {
        model_id.to_string()
    } else {
        format!("{provider}/{model_id}")
    })
}
#[cfg(test)]
#[path = "selection_policy_tests.rs"]
mod tests;
