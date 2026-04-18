//! Local CUDA model reference normalization and resolution.

use crate::tui::constants::AUTOCHAT_LOCAL_DEFAULT_MODEL;

/// Normalize a model ref for local CUDA: if no provider prefix, add one.
pub fn normalize_local_model_ref(model: &str) -> String {
    let trimmed = model.trim();
    if trimmed.is_empty() {
        return AUTOCHAT_LOCAL_DEFAULT_MODEL.to_string();
    }
    let (provider_name, _) = crate::provider::parse_model_string(trimmed);
    if provider_name.is_some() {
        trimmed.to_string()
    } else {
        format!("local_cuda/{trimmed}")
    }
}

/// Resolve the model for the local inference loop.
pub fn resolve_local_loop_model(current_model: Option<&str>) -> String {
    if let Ok(explicit) = std::env::var("CODETETHER_TUI_LOCAL_MODEL")
        .or_else(|_| std::env::var("CODETETHER_LOCAL_CUDA_MODEL"))
        .or_else(|_| std::env::var("LOCAL_CUDA_MODEL"))
    {
        let normalized = normalize_local_model_ref(&explicit);
        if !normalized.trim().is_empty() {
            return normalized;
        }
    }

    if let Some(model) = current_model {
        let normalized = normalize_local_model_ref(model);
        if normalized.starts_with("local_cuda/") {
            return normalized;
        }
    }

    AUTOCHAT_LOCAL_DEFAULT_MODEL.to_string()
}
