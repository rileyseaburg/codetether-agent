//! Request-boundary coverage for additional GPT-6 model variants.

use super::{build_converse_body, request};

#[test]
fn every_gpt6_variant_omits_temperature() {
    for model in ["openai.gpt-6-astral", "us.openai.gpt-6.1-sol-fast:xhigh"] {
        let body = build_converse_body(&request(model), model);
        assert!(
            body["inferenceConfig"].get("temperature").is_none(),
            "{model}"
        );
    }
}
