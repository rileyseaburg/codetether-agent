use super::{LEVELS, normalize, requires_reasoning};

#[test]
fn accepts_every_level_openrouter_enumerates() {
    // The router's 400 response lists exactly these values.
    for level in ["none", "minimal", "low", "medium", "high", "xhigh", "max"] {
        assert_eq!(normalize(level), Some(level), "{level} must be accepted");
    }
    assert_eq!(LEVELS.len(), 7);
}

#[test]
fn normalizes_case_and_surrounding_whitespace() {
    assert_eq!(normalize(" HIGH "), Some("high"));
    assert_eq!(normalize("Medium"), Some("medium"));
}

#[test]
fn rejects_values_openrouter_would_refuse() {
    // `ultra` is a Codex-only level; sending it here yields HTTP 400.
    assert_eq!(normalize("ultra"), None);
    assert_eq!(normalize("bogus"), None);
    assert_eq!(normalize(""), None);
}

#[test]
fn flags_grok_versions_that_mandate_reasoning() {
    assert!(requires_reasoning("x-ai/grok-4.6"));
    assert!(requires_reasoning("x-ai/grok-4.5"));
}

#[test]
fn older_grok_builds_still_allow_disabling_reasoning() {
    assert!(!requires_reasoning("x-ai/grok-4.3"));
    assert!(!requires_reasoning("x-ai/grok-4.20"));
    assert!(!requires_reasoning("openai/gpt-4o"));
}
