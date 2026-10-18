//! High-level formatting helpers that combine profiles and avatars.

use super::avatars::agent_avatar;
use crate::agent_profile;

/// One-line identity: `avatar @name ‹codename›`.
///
/// # Arguments
/// * `agent_name` - Name displayed alongside its avatar and codename.
///
/// # Returns
/// An owned, single-line agent label.
///
/// # Examples
/// ```
/// assert!(codetether_agent_identity::format_agent_identity("planner").contains("Strategist"));
/// ```
pub fn format_agent_identity(agent_name: &str) -> String {
    let profile = agent_profile(agent_name);
    format!(
        "{} @{} ‹{}›",
        agent_avatar(agent_name),
        agent_name,
        profile.codename
    )
}

/// Short summary: `codename — profile (personality)`.
///
/// # Arguments
/// * `agent_name` - Name used to select the persona.
///
/// # Returns
/// The codename, role and personality as one line.
///
/// # Examples
/// ```
/// assert!(codetether_agent_identity::format_agent_profile_summary("planner").starts_with("Strategist"));
/// ```
pub fn format_agent_profile_summary(agent_name: &str) -> String {
    let profile = agent_profile(agent_name);
    format!(
        "{} — {} ({})",
        profile.codename, profile.profile, profile.personality
    )
}
