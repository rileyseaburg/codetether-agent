//! Tests for model-specific request temperature selection.

use super::super::temperature_for;

#[test]
fn incompatible_models_do_not_get_a_default_temperature() {
    assert_eq!(temperature_for("bedrock/us.openai.gpt-6-astra"), None);
    assert_eq!(
        temperature_for("bedrock/us.anthropic.claude-haiku-5-5"),
        None
    );
    assert_eq!(temperature_for("bedrock/claude-haiku-4-5"), Some(0.7));
}
