//! Agent profile struct and per-agent profile lookup.

use super::fallback_profiles::*;
use super::named_profiles::*;

/// Identity metadata for a named agent.
#[derive(Debug, Clone)]
pub struct AgentProfile {
    pub codename: &'static str,
    pub profile: &'static str,
    pub personality: &'static str,
    pub collaboration_style: &'static str,
    pub signature_move: &'static str,
}

/// Return the [`AgentProfile`] for a known agent name, or a deterministic fallback.
pub fn agent_profile(agent_name: &str) -> AgentProfile {
    let n = agent_name.to_ascii_lowercase();
    if n.contains("planner") { return profile_strategist(); }
    if n.contains("research") { return profile_archivist(); }
    if n.contains("coder") || n.contains("implement") { return profile_forge(); }
    if n.contains("review") { return profile_sentinel(); }
    if n.contains("tester") || n.contains("test") { return profile_probe(); }
    if n.contains("integrat") { return profile_conductor(); }
    if n.contains("skeptic") || n.contains("risk") { return profile_radar(); }
    if n.contains("summary") || n.contains("summarizer") { return profile_beacon(); }
    fallback_profile(&n)
}

fn fallback_profile(name: &str) -> AgentProfile {
    let profiles = [
        profile_navigator(), profile_vector(), profile_signal(), profile_kernel(),
    ];
    let hash = fnv1a(name);
    profiles[hash as usize % profiles.len()].clone()
}

fn fnv1a(s: &str) -> u64 {
    let mut h: u64 = 2_166_136_261;
    for b in s.bytes() { h = (h ^ u64::from(b)).wrapping_mul(16_777_619); }
    h
}
