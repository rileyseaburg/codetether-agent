//! Build retained detail strings for bus log entries.

#[path = "entry_speech_payload.rs"]
mod speech_payload;
pub use speech_payload::build as speech;

/// Build retained detail for a message event.
/// # Arguments
/// Sender, recipient, transport flag, part count and message text.
/// # Returns
/// Formatted message metadata and bounded text.
/// # Examples
/// ```
/// let text = codetether_bus_view::entry_payload::message("a", "b", false, 1, "hello"); assert!(text.contains("hello"));
/// ```
pub fn message(from: &str, to: &str, a2a: bool, parts: usize, text: &str) -> String {
    let retained = codetether_view_retention::bus::detail(text, "bus message");
    format!(
        "From: {from}\nTo: {to}\nTransport: {}\nParts ({parts}):\n{retained}",
        if a2a { "A2A/mDNS peer" } else { "local bus" }
    )
}

/// Build retained detail for a tool response event.
/// # Arguments
/// Request, agent, step, tool, success flag and result.
/// # Returns
/// Formatted response metadata and bounded result.
/// # Examples
/// ```
/// let text = codetether_bus_view::entry_payload::tool_response("r", "a", 1, "read", true, "ok"); assert!(text.contains("ok"));
/// ```
pub fn tool_response(
    req: &str,
    agent: &str,
    step: usize,
    tool: &str,
    success: bool,
    result: &str,
) -> String {
    let retained = codetether_view_retention::bus::detail(result, "bus tool response");
    format!(
        "Request: {req}\nAgent: {agent}\nStep: {step}\nTool: {tool}\nSuccess: {success}\nResult: {retained}"
    )
}

/// Build retained detail for a tool full event.
/// # Arguments
/// Agent, tool, step, success flag and complete output.
/// # Returns
/// Bounded tool detail with a metadata header.
/// # Examples
/// ```
/// let text = codetether_bus_view::entry_payload::tool_full("a", "read", 1, true, "ok"); assert!(text.contains("ok"));
/// ```
pub fn tool_full(agent: &str, tool: &str, step: usize, success: bool, output: &str) -> String {
    let header =
        format!("Agent: {agent}\nTool: {tool}\nStep: {step}\nSuccess: {success}\n\n--- Output ---");
    codetether_view_retention::bus::tool_detail(&header, output, "bus tool output")
}

/// Build retained detail for a thinking event.
/// # Arguments
/// Agent, step and reasoning text.
/// # Returns
/// Formatted step metadata and bounded reasoning.
/// # Examples
/// ```
/// let text = codetether_bus_view::entry_payload::thinking("a", 1, "thinking"); assert!(text.contains("thinking"));
/// ```
pub fn thinking(agent: &str, step: usize, text: &str) -> String {
    let retained = codetether_view_retention::bus::detail(text, "bus thinking");
    format!("Agent: {agent}\nStep: {step}\n\n--- Reasoning ---\n{retained}")
}

/// Build retained detail for a transcript event.
/// # Arguments
/// Room, role, final flag and transcript text.
/// # Returns
/// Bounded transcript detail with room metadata.
/// # Examples
/// ```
/// let text = codetether_bus_view::entry_payload::transcript("room", "user", true, "hello"); assert!(text.contains("hello"));
/// ```
pub fn transcript(room: &str, role: &str, final_flag: bool, text: &str) -> String {
    let header = format!("Room: {room}\nRole: {role}\nFinal: {final_flag}");
    codetether_view_retention::bus::tool_detail(&header, text, "voice transcript")
}
