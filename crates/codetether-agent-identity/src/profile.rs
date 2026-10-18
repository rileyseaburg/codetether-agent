//! Agent persona metadata for display and introductions.

/// Identity metadata for a named agent.
///
/// # Examples
/// ```
/// let profile = codetether_agent_identity::agent_profile("planner");
/// assert_eq!(profile.codename, "Strategist");
/// ```
#[derive(Debug, Clone, Copy)]
pub struct AgentProfile {
    /// Short persona label.
    pub codename: &'static str,
    /// Primary role description.
    pub profile: &'static str,
    /// Tone and working temperament.
    pub personality: &'static str,
    /// How the persona interacts with teammates.
    pub collaboration_style: &'static str,
    /// Characteristic contribution to a task.
    pub signature_move: &'static str,
}
