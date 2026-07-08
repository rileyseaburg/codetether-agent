//! Tests for `additional_model_request_fields` construction.

use super::additional_model_request_fields;

#[test]
fn fable_fields_include_adaptive_thinking() {
    let fields = additional_model_request_fields("global.anthropic.claude-fable-5").unwrap();
    assert_eq!(fields["thinking"]["type"], "adaptive");
    assert_eq!(fields["output_config"]["effort"], "medium");
}

#[test]
fn bedrock_openai_gpt_fields_include_reasoning_effort() {
    for id in [
        "openai.gpt-5.6-sol",
        "openai.gpt-5.6-terra",
        "openai.gpt-5.6-luna",
    ] {
        let fields = additional_model_request_fields(id).unwrap();
        assert_eq!(fields["reasoning_effort"], "medium");
    }
}

#[test]
fn claude_models_do_not_get_reasoning_effort_field() {
    let fields =
        additional_model_request_fields("us.anthropic.claude-sonnet-4-6").unwrap_or_default();
    assert!(fields.get("reasoning_effort").is_none());
}
