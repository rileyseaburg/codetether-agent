//! Regression: Bedrock rejects Sonnet 5.x requests carrying `temperature`.

use crate::provider::bedrock::build_converse_body;
#[path = "inference_fixture.rs"]
mod fixture;
use fixture::request;

#[test]
fn sonnet_5_converse_omits_temperature() {
    for model in [
        "global.anthropic.claude-sonnet-5-5",
        "us.anthropic.claude-sonnet-5",
        "global.anthropic.claude-sonnet-5",
    ] {
        let body = build_converse_body(&request(model), model);
        assert!(
            body["inferenceConfig"].get("temperature").is_none(),
            "{model}: {body}"
        );
    }
}
