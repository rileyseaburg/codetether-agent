//! Compact swarm bus payloads before broadcasting.

pub fn thinking(input: &str) -> String {
    crate::bus::payload::bounded(input, "\n\n[truncated in swarm bus payload: thinking]")
}

pub fn tool_output(input: &str) -> String {
    crate::bus::payload::bounded(input, "\n\n[truncated in swarm bus payload: tool output]")
}
