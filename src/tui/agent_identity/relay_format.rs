//! Relay formatting: participant labels and handoff lines.

use super::format::format_agent_identity;

/// Format a relay participant for display (user → "[you]", agent → identity).
#[allow(dead_code)]
pub fn format_relay_participant(participant: &str) -> String {
    if participant.eq_ignore_ascii_case("user") {
        "[you]".to_string()
    } else {
        format_agent_identity(participant)
    }
}

/// Format a relay handoff line: `[relay ID • round N] from → to`.
#[allow(dead_code)]
pub fn format_relay_handoff_line(relay_id: &str, round: usize, from: &str, to: &str) -> String {
    format!(
        "[relay {relay_id} • round {round}] {} → {}",
        format_relay_participant(from),
        format_relay_participant(to)
    )
}
