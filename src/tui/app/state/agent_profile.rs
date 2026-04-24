//! Agent identity profile lookup.
//!
//! Maps agent names to codenames, collaboration styles, and signature moves
//! using keyword matching with FNV-1a hash fallback.

use crate::session::Session;

use super::profile_defs::{AgentProfile, *};

/// A spawned sub-agent with its own independent LLM session.
#[allow(dead_code)]
pub struct SpawnedAgent {
    /// User-facing name (e.g. "planner", "coder")
    pub name: String,
    /// System instructions for this agent
    pub instructions: String,
    /// Independent conversation session
    pub session: Session,
    /// Whether this agent is currently processing a message
    pub is_processing: bool,
}

/// Map an agent name to its codename profile.
pub fn agent_profile(agent_name: &str) -> AgentProfile {
    let normalized = agent_name.to_ascii_lowercase();

    if normalized.contains("planner") {
        return PROFILE_PLANNER;
    }
    if normalized.contains("research") {
        return PROFILE_RESEARCH;
    }
    if normalized.contains("coder") || normalized.contains("implement") {
        return PROFILE_CODER;
    }
    if normalized.contains("review") {
        return PROFILE_REVIEW;
    }
    if normalized.contains("tester") || normalized.contains("test") {
        return PROFILE_TESTER;
    }
    if normalized.contains("integrat") {
        return PROFILE_INTEGRATOR;
    }
    if normalized.contains("skeptic") || normalized.contains("risk") {
        return PROFILE_SKEPTIC;
    }
    if normalized.contains("summary") || normalized.contains("summarizer") {
        return PROFILE_SUMMARIZER;
    }

    // FNV-1a hash into fallback profiles
    let mut hash: u64 = 2_166_136_261;
    for byte in normalized.bytes() {
        hash = (hash ^ u64::from(byte)).wrapping_mul(16_777_619);
    }
    FALLBACK_PROFILES[hash as usize % FALLBACK_PROFILES.len()]
}
