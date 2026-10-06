//! Regression coverage for identity carried alongside a caller system string.

use super::{caller_prompt, system_prompt};
use crate::provider::metrics::identity::prompt;

#[test]
fn identity_system_prompt_preserves_caller_text() {
    let caller = "  Follow the caller's instructions.\n\nKeep spacing.  ";
    let result = system_prompt(caller, "candle", "local-model");
    assert_eq!(caller_prompt(&result), caller);
    assert_eq!(
        result,
        format!("{}\n\n{caller}", prompt("candle", "local-model"))
    );
}

#[test]
fn identity_system_prompt_replaces_stale_and_duplicate_blocks() {
    let stale = prompt("old-provider", "old-model");
    let original = format!("{stale}\n\n{stale}\n\nCaller prompt");
    let current = system_prompt(&original, "local_cuda", "loaded-model");
    assert_eq!(current.matches("<codetether-harness-identity>").count(), 1);
    assert_eq!(caller_prompt(&current), "Caller prompt");
    assert!(current.contains("loaded-model"));
    assert!(!current.contains("old-model"));
    assert_eq!(
        system_prompt(&current, "local_cuda", "loaded-model"),
        current
    );
}

#[test]
fn identity_system_prompt_does_not_strip_embedded_or_incomplete_blocks() {
    let embedded = format!("Caller text\n{}", prompt("quoted", "example"));
    let incomplete = "<codetether-harness-identity>\nCaller-owned fragment";
    let attached = format!("{}Caller suffix", prompt("quoted", "example"));
    for caller in [embedded.as_str(), incomplete, attached.as_str()] {
        assert_eq!(caller_prompt(caller), caller);
        assert!(system_prompt(caller, "current", "model").ends_with(caller));
    }
}

#[test]
fn identity_system_prompt_handles_an_empty_caller() {
    let identity = prompt("candle", "model");
    assert_eq!(system_prompt("", "candle", "model"), identity);
    assert_eq!(system_prompt(&identity, "candle", "model"), identity);
    assert_eq!(caller_prompt(&identity), "");
}
