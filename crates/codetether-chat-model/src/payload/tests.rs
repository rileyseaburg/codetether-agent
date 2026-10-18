//! Existing retained-payload bounds and envelope regression cases.

use super::*;

#[test]
fn output_is_bounded() {
    let text = "x".repeat(TOOL_OUTPUT_RETAINED_BYTES + 1);
    let bounded = tool_output(&text);
    assert!(bounded.len() < text.len());
    assert!(bounded.contains("tool output"));
}

#[test]
fn output_removes_model_feedback_envelope() {
    let text = "Tool call feedback\n- tool: bash\n\nOutput:\nreal output";
    assert_eq!(tool_output(text), "real output");
}
