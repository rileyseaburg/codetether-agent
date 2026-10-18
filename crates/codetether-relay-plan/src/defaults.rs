//! Default self-organizing relay team.
use crate::RelayProfile;

/// Build the default self-organizing relay team.
/// # Arguments
/// * `count` - Number of specialists to create.
/// # Returns
/// Profiles numbered from one, in creation order.
/// # Examples
/// ```
/// assert_eq!(codetether_relay_plan::build_relay_profiles(2).len(), 2);
/// ```
pub fn build_relay_profiles(count: usize) -> Vec<RelayProfile> {
    let mut profiles = Vec::with_capacity(count);
    for idx in 0..count {
        let name = format!("auto-agent-{}", idx + 1);

        let instructions = format!(
            "You are @{name}.\n\
             Role policy: self-organize from task context and current handoff instead of assuming a fixed persona.\n\
             Mission: advance the relay with concrete, high-signal next actions and clear ownership boundaries.\n\n\
             This is a protocol-first relay conversation. Treat the incoming handoff as authoritative context.\n\
             Keep your response concise, concrete, and useful for the next specialist.\n\
             Include one clear recommendation for what the next agent should do.\n\
             If the task scope is too large, explicitly call out missing specialties and handoff boundaries.",
        );
        let mut capabilities = vec!["generalist".to_string(), "self-organizing".to_string()];
        codetether_autochat_core::ensure_required_relay_capabilities(&mut capabilities);

        profiles.push(RelayProfile {
            name,
            instructions,
            capabilities,
        });
    }
    profiles
}
