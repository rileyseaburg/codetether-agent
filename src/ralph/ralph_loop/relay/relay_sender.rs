//! Determine the predecessor agent for one ordered relay turn.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) fn relay_sender(
        ordered_agents: &[String],
        idx: usize,
        round: usize,
    ) -> String {
        let from = if idx == 0 {
            if round == 1 {
                "user".to_string()
            } else {
                ordered_agents[ordered_agents.len() - 1].clone()
            }
        } else {
            ordered_agents[idx - 1].clone()
        };

        from
    }
}
