//! Shut down the relay agents and report the final handoff text.
use super::relay_types::RelayProgress;
use super::*;
impl RalphLoop {
    pub(super) fn finish_relay(
        relay: &ProtocolRelayRuntime,
        ordered_agents: &[String],
        progress: RelayProgress,
        story_id: &str,
    ) -> String {
        let RelayProgress {
            baton,
            convergence_hits,
            turns,
            ..
        } = progress;
        relay.shutdown_agents(ordered_agents);

        info!(
            story_id = %story_id,
            turns,
            convergence_hits,
            "Relay team completed"
        );

        baton
    }
}
