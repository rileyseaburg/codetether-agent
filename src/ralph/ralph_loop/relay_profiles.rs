//! Adapt the loop profile builder to the shared role definitions.
use super::*;
impl RalphLoop {
    /// Build default relay profiles for code implementation tasks.
    pub(super) fn build_implementation_relay_profiles(
        max_agents: usize,
        working_dir: &std::path::Path,
    ) -> Vec<(String, String, Vec<String>)> {
        codetether_ralph_core::relay_profiles::build_implementation_relay_profiles(
            max_agents,
            working_dir,
        )
    }
}
