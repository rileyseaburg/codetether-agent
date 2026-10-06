//! Credential-free native identity selection regressions.

use super::system_prompt;
use crate::provider::metrics::identity::{caller_prompt, prompt};

#[test]
fn thinker_identity_candle_uses_loaded_runtime_and_refreshes_switches() {
    let caller = "  Keep these instructions.\n";
    let stale = system_prompt(caller, "candle", "loaded-model-a");
    let current = system_prompt(&stale, "candle", "loaded-model-b");
    assert_eq!(
        current,
        format!("{}\n\n{caller}", prompt("candle", "loaded-model-b"))
    );
    assert_eq!(caller_prompt(&current), caller);
    assert_eq!(system_prompt(&current, "candle", "loaded-model-b"), current);
    assert!(!current.contains("loaded-model-a"));
}

#[test]
fn thinker_identity_local_cuda_uses_loaded_model_not_configured_alias() {
    let caller = "  Caller instructions.\n";
    let stale = system_prompt(caller, "local_cuda", "configured-alias");
    let current = system_prompt(&stale, "local_cuda", "loaded-model");
    assert_eq!(
        current,
        format!("{}\n\n{caller}", prompt("local_cuda", "loaded-model"))
    );
    assert_eq!(current.matches("<codetether-harness-identity>").count(), 1);
    assert!(!current.contains("configured-alias"));
    let switched = system_prompt(&current, "local_cuda", "loaded-model-b");
    assert_eq!(
        switched,
        format!("{}\n\n{caller}", prompt("local_cuda", "loaded-model-b"))
    );
    assert_eq!(caller_prompt(&switched), caller);
    assert!(!switched.contains("\"loaded-model\""));
    assert_eq!(switched.matches("<codetether-harness-identity>").count(), 1);
    assert_eq!(
        system_prompt(&switched, "local_cuda", "loaded-model-b"),
        switched
    );
}

#[test]
fn thinker_identity_native_preserves_embedded_caller_metadata() {
    let caller = format!("Keep this example:\n{}", prompt("example", "example-model"));
    let current = system_prompt(&caller, "candle", "loaded-model");
    assert_eq!(caller_prompt(&current), caller);
}
