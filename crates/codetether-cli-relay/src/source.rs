//! Select the preceding sender in the current team ordering.
use crate::{Host, state::Runtime};
impl<H: Host> Runtime<H> {
    pub(super) fn source(&self, round: usize, idx: usize) -> String {
        if idx == 0 {
            if round == 1 {
                "user".to_string()
            } else {
                self.team.ordered_agents[self.team.ordered_agents.len() - 1].clone()
            }
        } else {
            self.team.ordered_agents[idx - 1].clone()
        }
    }
}
