//! Tests for [`reset_threshold`](super::reset_threshold).
//!
//! Env-var precedence cases are *not* tested here because they mutate
//! process-global state; only the env-free paths are exercised.

use super::reset_threshold::resolve_for_model;
use crate::provider::limits::context_window_for_model;

#[test]
fn default_is_half_the_model_context_window() {
    // Only valid when no env overrides are set in the test process.
    if std::env::var("CODETETHER_CONTEXT_RESET_THRESHOLD_TOKENS").is_ok()
        || std::env::var("CODETETHER_CONTEXT_RESET_THRESHOLD_FRACTION").is_ok()
    {
        return;
    }
    let window = context_window_for_model("claude-sonnet-4");
    assert_eq!(resolve_for_model("claude-sonnet-4", None), window / 2);
}

#[test]
fn persisted_threshold_wins_over_default_fraction() {
    if std::env::var("CODETETHER_CONTEXT_RESET_THRESHOLD_TOKENS").is_ok()
        || std::env::var("CODETETHER_CONTEXT_RESET_THRESHOLD_FRACTION").is_ok()
    {
        return;
    }
    assert_eq!(resolve_for_model("gpt-4o", Some(42_000)), 42_000);
}

#[test]
fn zero_persisted_threshold_falls_back_to_fraction() {
    if std::env::var("CODETETHER_CONTEXT_RESET_THRESHOLD_TOKENS").is_ok()
        || std::env::var("CODETETHER_CONTEXT_RESET_THRESHOLD_FRACTION").is_ok()
    {
        return;
    }
    let window = context_window_for_model("gpt-4o");
    assert_eq!(resolve_for_model("gpt-4o", Some(0)), window / 2);
}

#[test]
fn larger_windows_get_proportionally_larger_thresholds() {
    if std::env::var("CODETETHER_CONTEXT_RESET_THRESHOLD_TOKENS").is_ok()
        || std::env::var("CODETETHER_CONTEXT_RESET_THRESHOLD_FRACTION").is_ok()
    {
        return;
    }
    let small = resolve_for_model("gpt-4o", None); // 128k window
    let large = resolve_for_model("gemini-2.5-pro", None); // 2M window
    assert!(large > small);
}
