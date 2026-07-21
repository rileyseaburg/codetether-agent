//! Tests for profile selection without mutating process environment state.

use super::{RequestedProfile, is_codex_provider, parse, resolve};

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
fn codex_provider_aliases_are_recognized() {
    assert!(is_codex_provider("openai-codex"));
    assert!(is_codex_provider("codex"));
    assert!(is_codex_provider("chatgpt"));
    assert!(!is_codex_provider("openai"));
}

#[test]
fn automatic_mode_is_codex_only_and_explicit_modes_win() {
    assert!(resolve(RequestedProfile::Automatic, "openai-codex"));
    assert!(!resolve(RequestedProfile::Automatic, "openai"));
    assert!(resolve(RequestedProfile::Coding, "anthropic"));
    assert!(!resolve(RequestedProfile::Full, "openai-codex"));
    assert!(!resolve(RequestedProfile::MuxManager, "openai-codex"));
}
