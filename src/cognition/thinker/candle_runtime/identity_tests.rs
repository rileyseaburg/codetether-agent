//! Credential-free tests of the snapshot used by both native loaders.

use super::{NativeModelIdentity, ThinkerConfig};
use crate::provider::metrics::identity::{caller_prompt, prompt, system_prompt};

#[test]
fn thinker_identity_native_snapshot_is_not_a_diagnostic_label() {
    let mut config = ThinkerConfig {
        model: "harness-model-a".into(),
        candle_model_path: Some("/models/checkpoint.gguf".into()),
        ..Default::default()
    };
    let loaded = NativeModelIdentity::from_config(&config);
    config.model = "harness-model-b".into();
    assert_eq!(loaded.model(), "harness-model-a");
    let switched = NativeModelIdentity::from_config(&config);
    assert_eq!(switched.model(), "harness-model-b");
    let caller = "  Keep my instructions.\n";
    let old = system_prompt(caller, "candle", loaded.model());
    let current = system_prompt(&old, "local_cuda", switched.model());
    assert_eq!(
        current,
        format!("{}\n\n{caller}", prompt("local_cuda", "harness-model-b"))
    );
    assert_eq!(caller_prompt(&current), caller);
    assert!(!current.contains("harness-model-a"));
    assert!(!current.contains("checkpoint.gguf"));
    assert_eq!(current.matches("<codetether-harness-identity>").count(), 1);
}
