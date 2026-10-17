//! Environment override for A2A session models.
pub(super) fn model_from_env() -> Option<String> {
    std::env::var("CODETETHER_DEFAULT_MODEL")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}
