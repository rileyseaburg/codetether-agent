//! Provider transport-routing regression.
use crate::provider::BedrockProvider;

#[test]
fn fable_uses_invoke_model_adapter() {
    assert!(BedrockProvider::should_use_invoke_model(
        "us.anthropic.claude-fable-5"
    ));
    assert!(!BedrockProvider::should_use_invoke_model(
        "us.anthropic.claude-sonnet-4-20250514-v1:0"
    ));
}
