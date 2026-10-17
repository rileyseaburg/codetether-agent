//! Regression for Bedrock's rejection of temperature on Claude Haiku 5.5.

use crate::build_converse_body;
#[path = "inference_fixture.rs"]
mod fixture;
use fixture::request;

#[test]
fn haiku_55_converse_omits_even_explicit_temperature() {
    let model = "us.anthropic.claude-haiku-5-5";
    let body = build_converse_body(&request(model), model);
    assert!(
        body["inferenceConfig"].get("temperature").is_none(),
        "{body}"
    );
}
