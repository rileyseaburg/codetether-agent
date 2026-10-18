//! Compact swarm bus payloads before broadcasting.

/// Bound a swarm thinking payload with its original truncation hint.
/// # Arguments
/// * `input` - Text ready for cross-agent broadcast.
/// # Returns
/// Bounded text using the shared bus byte limit.
/// # Examples
/// ```
/// assert_eq!(codetether_swarm_bus::thinking("small"), "small");
/// ```
pub fn thinking(input: &str) -> String {
    codetether_bus_core::payload::bounded(input, "\n\n[truncated in swarm bus payload: thinking]")
}

/// Bound a swarm tool output payload with its original truncation hint.
/// # Arguments
/// * `input` - Text ready for cross-agent broadcast.
/// # Returns
/// Bounded text using the shared bus byte limit.
/// # Examples
/// ```
/// assert_eq!(codetether_swarm_bus::tool_output("small"), "small");
/// ```
pub fn tool_output(input: &str) -> String {
    codetether_bus_core::payload::bounded(
        input,
        "\n\n[truncated in swarm bus payload: tool output]",
    )
}
