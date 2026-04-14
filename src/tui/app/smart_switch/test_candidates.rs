//! Tests for candidate generation.

use super::*;
use std::collections::HashSet;

#[test]
fn preferred_models_returns_list_for_known_providers() {
    let models = smart_switch_preferred_models("minimax");
    assert!(!models.is_empty());
    assert!(models.contains(&"MiniMax-M2.5"));
    let models = smart_switch_preferred_models("zai");
    assert!(models.contains(&"glm-5"));
}

#[test]
fn preferred_models_returns_empty_for_unknown_provider() {
    assert!(smart_switch_preferred_models("unknown-provider").is_empty());
}

#[test]
fn candidates_prioritizes_same_provider() {
    let available = vec!["minimax".to_string(), "openai".to_string()];
    let attempted = HashSet::new();
    let candidates = smart_switch_candidates(
        Some("minimax/MiniMax-M2"),
        Some("minimax"),
        &available,
        &attempted,
    );
    assert!(candidates.first().unwrap().starts_with("minimax/"));
}

#[test]
fn candidates_excludes_attempted_models() {
    let available = vec!["minimax".to_string(), "openai".to_string()];
    let mut attempted = HashSet::new();
    attempted.insert(smart_switch_model_key("minimax/MiniMax-M2.5"));
    let candidates = smart_switch_candidates(
        Some("minimax/MiniMax-M2.5"),
        Some("minimax"),
        &available,
        &attempted,
    );
    assert!(!candidates.iter().any(|c| c == "minimax/MiniMax-M2.5"));
}
