//! Existing model selection regression coverage.
use super::*;

#[test]
fn copilot_prefers_gpt_5_mini_when_available() {
    let models = vec![
        model("gpt-4o"),
        model("gpt-5-mini"),
        model("claude-sonnet-4"),
    ];
    assert_eq!(choose_copilot_model(&models).as_deref(), Some("gpt-5-mini"));
}
