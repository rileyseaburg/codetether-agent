//! Shut down the relay agents and report the final handoff text.
use super::relay_types::RelayProgress;
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) fn finish_relay(
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

        info!(target: "codetether_agent::ralph::ralph_loop::relay_finish",
            story_id = %story_id,
            turns,
            convergence_hits,
            "Relay team completed"
        );

        baton
    }
}
