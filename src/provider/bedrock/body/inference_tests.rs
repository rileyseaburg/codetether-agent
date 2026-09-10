//! Request-boundary regression for Bedrock's rejection of Astra temperature.

use crate::provider::bedrock::build_converse_body;
#[path = "inference_fixture.rs"]
mod fixture;
use fixture::request;

#[test]
fn astra_converse_omits_even_explicit_temperature() {
    for model in [
        "gpt-6-astra",
        "openai.gpt-6-astra",
        "us.openai.gpt-6-astra",
        "bedrock/us.openai.gpt-6-astra",
        "eu.openai.gpt-6-astra",
        "apac.openai.gpt-6-astra",
        "global.openai.gpt-6-astra",
        "bedrock/us.openai.gpt-6-astra-fast:xhigh",
        "US.OPENAI.GPT-6-ASTRA",
        "arn:aws:bedrock:us-east-1:123456789012:inference-profile/us.openai.gpt-6-astra",
    ] {
        let body = build_converse_body(&request(model), model);
        assert!(
            body["inferenceConfig"].get("temperature").is_none(),
            "{model}: {body}"
        );
        assert_eq!(body["inferenceConfig"]["maxTokens"], 128);
    }
}

#[test]
fn supported_models_keep_temperature_and_other_controls() {
    for model in [
        "openai.gpt-oss-120b",
        "amazon.nova-pro-v1:0",
        "us.anthropic.claude-sonnet-4-20250514-v1:0",
        "openai.gpt-6-astral",
    ] {
        let mut request = request(model);
        request.top_p = Some(0.5);
        let config = &build_converse_body(&request, model)["inferenceConfig"];
        assert!(config.get("temperature").is_some(), "{model}");
        assert_eq!(config["topP"], 0.5);
        assert_eq!(config["maxTokens"], 128);
    }
}

#[test]
fn claude_temperature_omission_is_preserved() {
    let model = "us.anthropic.claude-opus-4-7";
    let body = build_converse_body(&request(model), model);
    assert!(body["inferenceConfig"].get("temperature").is_none());
}
