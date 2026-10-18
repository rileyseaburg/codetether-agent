//! Original provider policy regression case.
use crate::temperature_is_deprecated;

#[test]
fn detects_newer_families_that_reject_temperature() {
    for model in [
        "bedrock/us.openai.gpt-6-astra",
        "us.anthropic.claude-fable-5",
        "global.anthropic.claude-sonnet-5-5",
        "us.anthropic.claude-haiku-5-5",
        "claude-5-haiku",
    ] {
        assert!(temperature_is_deprecated(model), "{model}");
    }
}
