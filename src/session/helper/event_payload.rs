//! Bounded payloads for live UI/session events.

use crate::util::truncate_bytes_safe;

const TOOL_OUTPUT_EVENT_MAX_BYTES: usize = 8 * 1024;
const TOOL_ARGUMENTS_EVENT_MAX_BYTES: usize = 4 * 1024;
const TRUNCATED_OUTPUT_MARKER: &str =
    "\n\n[truncated for live TUI event; full output remains in session history]";
const TRUNCATED_ARGUMENTS_MARKER: &str = "\n\n[truncated for live TUI event: tool arguments]";

pub(crate) fn bounded_tool_output(output: &str) -> String {
    bounded(output, TOOL_OUTPUT_EVENT_MAX_BYTES, TRUNCATED_OUTPUT_MARKER)
}

pub(crate) fn bounded_tool_arguments(arguments: &str) -> String {
    bounded(
        arguments,
        TOOL_ARGUMENTS_EVENT_MAX_BYTES,
        TRUNCATED_ARGUMENTS_MARKER,
    )
}

fn bounded(input: &str, max_bytes: usize, marker: &str) -> String {
    if input.len() <= max_bytes {
        return input.to_string();
    }
    let mut bounded = truncate_bytes_safe(input, max_bytes).to_string();
    bounded.push_str(marker);
    bounded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_small_output() {
        assert_eq!(bounded_tool_output("ok"), "ok");
    }

    #[test]
    fn truncates_large_output() {
        let text = "a".repeat(TOOL_OUTPUT_EVENT_MAX_BYTES + 1);
        let bounded = bounded_tool_output(&text);
        assert!(bounded.len() < text.len());
        assert!(bounded.contains("truncated for live TUI event"));
    }

    #[test]
    fn truncates_large_arguments() {
        let text = "b".repeat(TOOL_ARGUMENTS_EVENT_MAX_BYTES + 1);
        let bounded = bounded_tool_arguments(&text);
        assert!(bounded.len() < text.len());
        assert!(bounded.contains("tool arguments"));
    }
}
