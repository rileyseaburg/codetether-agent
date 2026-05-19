//! Helpers for compact live bus payloads from prompt loops.

pub fn compact(input: &str, label: &str) -> String {
    crate::bus::payload::bounded(input, label)
}

pub fn compact_tool(input: &str) -> String {
    compact(input, "\n\n[truncated in session bus payload: tool output]")
}

pub fn compact_thinking(input: &str) -> String {
    compact(input, "\n\n[truncated in session bus payload: thinking]")
}
