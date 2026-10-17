//! Tests for profile selection without mutating process environment state.

use super::{RequestedProfile, parse, provider_rules, resolve};

#[test]
fn profile_aliases_parse_to_explicit_modes() {
    assert_eq!(parse(Some("lean")), RequestedProfile::Coding);
    assert_eq!(parse(Some("CODEX")), RequestedProfile::Coding);
    assert_eq!(parse(Some("full")), RequestedProfile::Full);
    assert_eq!(parse(Some("mux-manager")), RequestedProfile::MuxManager);
    assert_eq!(parse(None), RequestedProfile::Automatic);
    assert_eq!(parse(Some("future-profile")), RequestedProfile::Unknown);
}

#[test]
fn compact_provider_aliases_are_recognized() {
    assert!(provider_rules::prefers_coding("openai-codex"));
    assert!(provider_rules::prefers_coding("codex"));
    assert!(provider_rules::prefers_coding("chatgpt"));
    assert!(!provider_rules::prefers_coding("openrouter"));
    assert!(!provider_rules::prefers_coding("openai"));
}

#[test]
fn automatic_mode_uses_discovery_for_web_routed_providers() {
    assert!(resolve(RequestedProfile::Automatic, "openai-codex"));
    assert!(!resolve(RequestedProfile::Automatic, "openrouter"));
    assert!(provider_rules::resolve_discovery(
        RequestedProfile::Automatic,
        "openrouter"
    ));
    assert!(provider_rules::resolve_discovery(
        RequestedProfile::Automatic,
        "gemini-web"
    ));
    assert!(!provider_rules::resolve_discovery(
        RequestedProfile::Full,
        "gemini-web"
    ));
    assert!(!resolve(RequestedProfile::Automatic, "openai"));
    assert!(resolve(RequestedProfile::Coding, "anthropic"));
    assert!(!resolve(RequestedProfile::Full, "openai-codex"));
    assert!(!resolve(RequestedProfile::MuxManager, "openai-codex"));
}
