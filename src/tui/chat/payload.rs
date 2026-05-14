//! Bounded payloads retained by TUI chat messages.

use crate::util::truncate_bytes_safe;

const TOOL_OUTPUT_RETAINED_BYTES: usize = 8 * 1024;
const TOOL_ARGUMENTS_RETAINED_BYTES: usize = 4 * 1024;

/// Bound tool output retained in the TUI message list.
pub fn tool_output(output: &str) -> String {
    bounded(output, TOOL_OUTPUT_RETAINED_BYTES, "tool output")
}

/// Bound tool call arguments retained in the TUI message list.
pub fn tool_arguments(arguments: &str) -> String {
    bounded(arguments, TOOL_ARGUMENTS_RETAINED_BYTES, "tool arguments")
}

fn bounded(input: &str, max_bytes: usize, label: &str) -> String {
    if input.len() <= max_bytes {
        return input.to_string();
    }
    let mut bounded = truncate_bytes_safe(input, max_bytes).to_string();
    bounded.push_str("\n\n[truncated in TUI retained message: ");
    bounded.push_str(label);
    bounded.push(']');
    bounded
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
}
