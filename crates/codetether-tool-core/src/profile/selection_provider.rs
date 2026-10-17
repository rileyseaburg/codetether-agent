//! Provider-specific defaults for automatic tool-profile selection.

use super::RequestedProfile;

pub(super) fn resolve_discovery(requested: RequestedProfile, provider: &str) -> bool {
    requested == RequestedProfile::Automatic && matches!(provider, "gemini-web" | "openrouter")
}

pub(super) fn prefers_coding(provider: &str) -> bool {
    matches!(provider, "openai-codex" | "codex" | "chatgpt")
}
