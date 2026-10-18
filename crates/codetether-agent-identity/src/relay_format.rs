//! Relay formatting: participant labels and handoff lines.

use super::format::format_agent_identity;

/// Format a relay participant for display (user → "[you]", agent → identity).
#[allow(dead_code)]
///
/// # Arguments
/// * `participant` - Agent name or the special user label.
///
/// # Returns
/// The user marker or formatted agent identity.
///
/// # Examples
/// ```
/// assert_eq!(codetether_agent_identity::format_relay_participant("USER"), "[you]");
/// ```
pub fn format_relay_participant(participant: &str) -> String {
    if participant.eq_ignore_ascii_case("user") {
        "[you]".to_string()
    } else {
        format_agent_identity(participant)
    }
}

/// Format a relay handoff line: `[relay ID • round N] from → to`.
#[allow(dead_code)]
///
/// # Arguments
/// * `relay_id` - Relay identifier.
/// * `round` - Current round number.
/// * `from` - Sending participant.
/// * `to` - Receiving participant.
///
/// # Returns
/// An owned relay handoff label.
///
/// # Examples
/// ```
/// assert!(codetether_agent_identity::format_relay_handoff_line("r1", 2, "user", "planner").starts_with("[relay r1 • round 2]"));
/// ```
pub fn format_relay_handoff_line(relay_id: &str, round: usize, from: &str, to: &str) -> String {
    format!(
        "[relay {relay_id} • round {round}] {} → {}",
        format_relay_participant(from),
        format_relay_participant(to)
    )
}
