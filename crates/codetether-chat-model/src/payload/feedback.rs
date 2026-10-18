//! Strip the model-facing feedback envelope from displayed tool output.

const TOOL_FEEDBACK_OUTPUT_MARKER: &str = "\n\nOutput:\n";

pub(super) fn strip_tool_feedback(output: &str) -> &str {
    if !output.starts_with("Tool call feedback\n") {
        return output;
    }
    output
        .split_once(TOOL_FEEDBACK_OUTPUT_MARKER)
        .map(|(_, body)| body)
        .unwrap_or(output)
}
