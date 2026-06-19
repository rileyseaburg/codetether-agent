//! Build retained detail strings for bus log entries.

pub fn message(from: &str, to: &str, a2a: bool, parts: usize, text: &str) -> String {
    let retained = crate::tui::bus_log_payload::detail(text, "bus message");
    format!(
        "From: {from}\nTo: {to}\nTransport: {}\nParts ({parts}):\n{retained}",
        if a2a { "A2A/mDNS peer" } else { "local bus" }
    )
}

pub fn tool_response(
    req: &str,
    agent: &str,
    step: usize,
    tool: &str,
    success: bool,
    result: &str,
) -> String {
    let retained = crate::tui::bus_log_payload::detail(result, "bus tool response");
    format!(
        "Request: {req}\nAgent: {agent}\nStep: {step}\nTool: {tool}\nSuccess: {success}\nResult: {retained}"
    )
}

pub fn tool_full(agent: &str, tool: &str, step: usize, success: bool, output: &str) -> String {
    let header =
        format!("Agent: {agent}\nTool: {tool}\nStep: {step}\nSuccess: {success}\n\n--- Output ---");
    crate::tui::bus_log_payload::tool_detail(&header, output, "bus tool output")
}

pub fn thinking(agent: &str, step: usize, text: &str) -> String {
    let retained = crate::tui::bus_log_payload::detail(text, "bus thinking");
    format!("Agent: {agent}\nStep: {step}\n\n--- Reasoning ---\n{retained}")
}

pub fn transcript(room: &str, role: &str, final_flag: bool, text: &str) -> String {
    let header = format!("Room: {room}\nRole: {role}\nFinal: {final_flag}");
    crate::tui::bus_log_payload::tool_detail(&header, text, "voice transcript")
}

/// Build (kind, summary, detail, color) for an `AgentSpeech` bus entry.
pub fn speech(
    act: &str,
    from: &str,
    to: &str,
    conversation_id: &str,
    content: &str,
) -> (String, String, String, ratatui::style::Color) {
    (
        format!("SAY•{act}"),
        format!("{from} → {to}: {content}"),
        format!("Act: {act}\nFrom: {from}\nTo: {to}\nConversation: {conversation_id}\n\n{content}"),
        ratatui::style::Color::Magenta,
    )
}
