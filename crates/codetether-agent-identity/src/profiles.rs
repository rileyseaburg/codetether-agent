//! Recognized role-name lookup with deterministic fallback.
use crate::AgentProfile;
use crate::fallback_select::fallback_profile;
use crate::named_delivery::*;
use crate::named_planning::*;

/// Return the [`AgentProfile`] for a known agent name, or a deterministic fallback.
///
/// # Arguments
/// * `agent_name` - Name whose role keywords choose the persona.
///
/// # Returns
/// Recognized persona metadata or a deterministic fallback.
///
/// # Examples
/// ```
/// assert_eq!(codetether_agent_identity::agent_profile("planner").codename, "Strategist");
/// ```
pub fn agent_profile(agent_name: &str) -> AgentProfile {
    let n = agent_name.to_ascii_lowercase();
    if n.contains("planner") {
        return profile_strategist();
    }
    if n.contains("research") {
        return profile_archivist();
    }
    if n.contains("coder") || n.contains("implement") {
        return profile_forge();
    }
    if n.contains("review") {
        return profile_sentinel();
    }
    if n.contains("tester") || n.contains("test") {
        return profile_probe();
    }
    if n.contains("integrat") {
        return profile_conductor();
    }
    if n.contains("skeptic") || n.contains("risk") {
        return profile_radar();
    }
    if n.contains("summary") || n.contains("summarizer") {
        return profile_beacon();
    }
    fallback_profile(&n)
}
