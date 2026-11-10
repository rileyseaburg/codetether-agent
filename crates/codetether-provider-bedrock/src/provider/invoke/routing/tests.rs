//! Exact streaming opt-in without changing older models or blocking calls.
use crate::provider::BedrockProvider;

#[test]
fn fable_52_streams_through_converse() {
    let model = BedrockProvider::resolve_model_id("us.anthropic.claude-fable-5-2");
    assert!(!BedrockProvider::should_use_invoke_model_stream(model));
    assert!(BedrockProvider::should_use_invoke_model(model));
}

#[test]
fn unverified_fable_profiles_keep_their_native_route() {
    for model in [
        "fable",
        "us.anthropic.claude-fable-5",
        "us.anthropic.claude-fable-5-1",
        "global.anthropic.claude-fable-5-2",
        "us.anthropic.claude-fable-5-20",
        "us.anthropic.claude-fable-5-2-preview",
        "us.anthropic.claude-fable-5-2-v2:0",
        "arn:aws:bedrock:us-east-1:123456789012:application-inference-profile/custom",
    ] {
        let resolved = BedrockProvider::resolve_model_id(model);
        assert_eq!(
            BedrockProvider::should_use_invoke_model_stream(resolved),
            BedrockProvider::should_use_invoke_model(resolved),
            "{model}"
        );
    }
}

#[test]
fn opus_stream_route_is_unchanged() {
    for model in ["us.anthropic.claude-opus-5-5", "amazon.nova-pro-v1:0"] {
        assert!(!BedrockProvider::should_use_invoke_model_stream(model));
        assert!(!BedrockProvider::should_use_invoke_model(model));
    }
}
