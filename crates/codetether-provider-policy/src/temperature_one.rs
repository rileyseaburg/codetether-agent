//! Existing model-name policy for temperature one.

/// Apply the established prefers temperature one policy.
/// # Arguments
/// Receives the provider, request or error inputs indicated by the signature.
/// # Returns
/// The policy decision without invoking providers or mutating session state.
/// # Examples
/// ```
/// assert!(codetether_provider_policy::prefers_temperature_one("kimi-k2"));
/// ```
pub fn prefers_temperature_one(model: &str) -> bool {
    let normalized = model.to_ascii_lowercase();
    normalized.contains("kimi-k2")
        || normalized.contains("glm-")
        || normalized.contains("minimax")
        || normalized.contains("deepseek")
}
