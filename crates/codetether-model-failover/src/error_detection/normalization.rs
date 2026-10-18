//! Provider and model identifier normalization.

/// Normalizes provider aliases (zhipuai/z-ai -> zai).
///
/// # Arguments
/// * `provider_name` — provider name input.
///
/// # Returns
/// The normalized value or policy result described above.
///
/// # Examples
/// ```
/// assert_eq!(codetether_model_failover::normalize_provider_alias("z-ai"), "zai");
/// ```
pub fn normalize_provider_alias(provider_name: &str) -> &str {
    if provider_name.eq_ignore_ascii_case("zhipuai") || provider_name.eq_ignore_ascii_case("z-ai") {
        "zai"
    } else {
        provider_name
    }
}

/// Normalizes a model reference for use as a key in the attempted models set.
///
/// # Arguments
/// * `model_ref` — model ref input.
///
/// # Returns
/// The normalized value or policy result described above.
///
/// # Examples
/// ```
/// assert_eq!(codetether_model_failover::smart_switch_model_key(" X/Y "), "x/y");
/// ```
pub fn smart_switch_model_key(model_ref: &str) -> String {
    model_ref.trim().to_ascii_lowercase()
}
