//! Environment configuration for provider registration.

pub(super) fn usai_base_url_from_env() -> Option<String> {
    [
        "CODETETHER_USAI_BASE_URL",
        "CODETETHER_USAI_API_BASE_URL",
        "USAI_BASE_URL",
        "USAI_API_BASE_URL",
    ]
    .into_iter()
    .find_map(|name| {
        std::env::var(name)
            .ok()
            .filter(|value| !value.trim().is_empty())
    })
}
