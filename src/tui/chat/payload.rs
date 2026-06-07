//! Bounded payloads retained by TUI chat messages.

use crate::util::truncate_bytes_safe;

const TOOL_OUTPUT_RETAINED_BYTES: usize = 8 * 1024;
const TOOL_ARGUMENTS_RETAINED_BYTES: usize = 4 * 1024;
const TOOL_FEEDBACK_OUTPUT_MARKER: &str = "\n\nOutput:\n";

/// Bound tool output retained in the TUI message list.
pub fn tool_output(output: &str) -> String {
    bounded(
        strip_tool_feedback(output),
        TOOL_OUTPUT_RETAINED_BYTES,
        "tool output",
    )
}

/// Bound tool call arguments retained in the TUI message list.
pub fn tool_arguments(arguments: &str) -> String {
    bounded(arguments, TOOL_ARGUMENTS_RETAINED_BYTES, "tool arguments")
}

fn bounded(input: &str, max_bytes: usize, label: &str) -> String {
    if input.len() <= max_bytes {
        return input.to_string();
    }
    let marker = format!("\n\n[truncated in TUI retained message: {label}]");
    let content_bytes = max_bytes.saturating_sub(marker.len());
    let mut bounded = truncate_bytes_safe(input, content_bytes).to_string();
    bounded.push_str(&marker);
    bounded
}

fn strip_tool_feedback(output: &str) -> &str {
    if !output.starts_with("Tool call feedback\n") {
        return output;
    }
    output
        .split_once(TOOL_FEEDBACK_OUTPUT_MARKER)
        .map(|(_, body)| body)
        .unwrap_or(output)
}

#[cfg(test)]
mod tests {
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
}
