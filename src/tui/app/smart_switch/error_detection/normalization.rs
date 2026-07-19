//! Provider and model identifier normalization.

/// Normalizes provider aliases (zhipuai/z-ai -> zai).
pub fn normalize_provider_alias(provider_name: &str) -> &str {
    if provider_name.eq_ignore_ascii_case("zhipuai") || provider_name.eq_ignore_ascii_case("z-ai") {
        "zai"
    } else {
        provider_name
    }
}

/// Normalizes a model reference for use as a key in the attempted models set.
pub fn smart_switch_model_key(model_ref: &str) -> String {
    model_ref.trim().to_ascii_lowercase()
}
