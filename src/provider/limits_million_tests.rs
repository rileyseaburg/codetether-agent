//! Tests for [`crate::provider::limits_million::has_million_token_context`].

use super::super::context_window_for_model;
use super::has_million_token_context;

#[test]
fn opus_5_has_million_token_context() {
    assert!(has_million_token_context("claude-opus-5"));
    assert!(has_million_token_context("global.anthropic.claude-opus-5"));
    assert!(has_million_token_context("vertex-anthropic/claude-opus-5"));
    assert!(has_million_token_context("Claude-Opus-5"));
}

#[test]
fn opus_47_and_glm_52_still_match() {
    assert!(has_million_token_context("claude-opus-4-7"));
    assert!(has_million_token_context("claude-opus-4.7"));
    assert!(has_million_token_context("glm-5.2"));
}

#[test]
fn older_models_do_not_match() {
    assert!(!has_million_token_context("claude-opus-4-6"));
    assert!(!has_million_token_context("claude-sonnet-4-6"));
}

#[test]
fn context_window_reflects_opus_5() {
    assert_eq!(context_window_for_model("claude-opus-5"), 1_000_000);
    assert_eq!(context_window_for_model("claude-opus-4-20250514"), 200_000);
}
