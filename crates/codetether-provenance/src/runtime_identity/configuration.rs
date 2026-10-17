//! Explicit process identity configuration.
pub(super) fn configured_identity() -> Option<String> {
    std::env::var("CODETETHER_AGENT_IDENTITY_ID")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}
