//! Initial bus registrations derived from planned profiles.
use codetether_bus_core::relay::RelayAgentProfile;
use codetether_relay_plan::RelayProfile;
pub(super) fn registrations(profiles: &[RelayProfile]) -> Vec<RelayAgentProfile> {
    profiles
        .iter()
        .map(|profile| RelayAgentProfile {
            name: profile.name.clone(),
            capabilities: profile.capabilities.clone(),
        })
        .collect()
}
