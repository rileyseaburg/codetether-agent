//! High-level formatting helpers that combine profiles and avatars.

use super::avatars::agent_avatar;
use super::profiles::{AgentProfile, agent_profile};

/// One-line identity: `avatar @name ‹codename›`.
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
pub fn format_agent_profile_summary(agent_name: &str) -> String {
    let profile = agent_profile(agent_name);
    format!(
        "{} — {} ({})",
        profile.codename, profile.profile, profile.personality
    )
}
