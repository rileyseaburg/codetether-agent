//! Native identity must refresh both provider and loaded model on every request.

use super::system_prompt;
use crate::provider::metrics::identity::{caller_prompt, prompt};

#[test]
fn thinker_identity_native_refreshes_provider_and_loaded_model_together() {
    let caller = "  Keep caller instructions and whitespace.\n";
    let candle = system_prompt(caller, "candle", "loaded-model-a");
    let cuda = system_prompt(&candle, "local_cuda", "loaded-model-b");
    assert_eq!(
        cuda,
        format!("{}\n\n{caller}", prompt("local_cuda", "loaded-model-b"))
    );
    assert_eq!(caller_prompt(&cuda), caller);
    assert_eq!(cuda.matches("<codetether-harness-identity>").count(), 1);
    assert!(!cuda.contains("loaded-model-a"));
    assert!(!cuda.contains("\"candle\""));

    let restored = system_prompt(&cuda, "candle", "loaded-model-c");
    assert_eq!(
        restored,
        format!("{}\n\n{caller}", prompt("candle", "loaded-model-c"))
    );
    assert_eq!(caller_prompt(&restored), caller);
    assert!(!restored.contains("loaded-model-b"));
    assert!(!restored.contains("local_cuda"));
}
