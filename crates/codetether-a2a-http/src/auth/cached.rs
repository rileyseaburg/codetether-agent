//! Lazy process-wide configured A2A bearer token.

/// Cached auth token. Read once from the environment to avoid the
/// per-request `env::var` global lock; subsequent calls hit the
/// process-local `OnceLock`.
static AUTH_TOKEN: std::sync::OnceLock<Option<String>> = std::sync::OnceLock::new();

pub(super) fn cached_token() -> Option<&'static str> {
    AUTH_TOKEN
        .get_or_init(|| {
            std::env::var("CODETETHER_AUTH_TOKEN")
                .ok()
                .filter(|s| !s.is_empty())
        })
        .as_deref()
}
