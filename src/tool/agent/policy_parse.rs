//! Model reference parsing for agent spawn policy.
//!
//! This module trims and splits model references into provider and model
//! identifiers for eligibility evaluation.
//!
//! # Examples
//!
//! ```ignore
//! let parts = parse_model_parts("openrouter/qwen/qwen3");
//! assert!(parts.is_some());
//! ```

use crate::provider::parse_model_string;

#[cfg(test)]
/// Normalizes a model reference for policy assertions.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(normalize_model(" GLM-5 "), "glm-5");
/// ```
pub(super) fn normalize_model(model: &str) -> String {
    model.trim().to_ascii_lowercase()
}

/// Splits a model reference into optional provider and model id parts.
///
/// # Examples
///
/// ```ignore
/// let parts = parse_model_parts("zai/glm-5");
/// assert!(parts.is_some());
/// ```
pub(super) fn parse_model_parts(model: &str) -> Option<(Option<String>, &str)> {
    let trimmed = model.trim();
    if trimmed.is_empty() {
        return None;
    }
    let (provider, model_id) = parse_model_string(trimmed);
    Some((provider.map(str::to_string), model_id))
}
