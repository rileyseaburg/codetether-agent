//! Local CUDA model reference normalization and resolution.

use crate::constants::AUTOCHAT_LOCAL_DEFAULT_MODEL;

/// Normalize a model ref for local CUDA: if no provider prefix, add one.
///
/// # Arguments
/// * `model` — model input.
///
/// # Returns
/// The normalized value or policy result described above.
///
/// # Examples
/// ```
/// assert_eq!(codetether_model_failover::local_model::normalize_local_model_ref("x"), "local_cuda/x");
/// ```
pub fn normalize_local_model_ref(model: &str) -> String {
    let trimmed = model.trim();
    if trimmed.is_empty() {
        return AUTOCHAT_LOCAL_DEFAULT_MODEL.to_string();
    }
    let (provider_name, _) = codetether_provider_core::routing::parse_model_string(trimmed);
    if provider_name.is_some() {
        trimmed.to_string()
    } else {
        format!("local_cuda/{trimmed}")
    }
}

/// Resolve the model for the local inference loop.
///
/// # Arguments
/// * `current_model` — current model input.
///
/// # Returns
/// The normalized value or policy result described above.
///
/// # Examples
/// ```
/// assert!(!codetether_model_failover::local_model::resolve_local_loop_model(None).is_empty());
/// ```
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
