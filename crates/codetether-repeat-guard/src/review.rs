//! A durable review clock independent of command and output novelty.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub(super) struct Review {
    pub results: u32,
    pub failures: u32,
    pub repeated: u32,
    pub first_turn: usize,
    pub last_turn: usize,
}

impl Review {
    pub fn observe(&mut self, turn: usize, failed: bool, repeated: bool) {
        if self.results == 0 {
            self.first_turn = turn;
        }
        self.last_turn = turn;
        self.results = self.results.saturating_add(1);
        self.failures = self.failures.saturating_add(u32::from(failed));
        self.repeated = self.repeated.saturating_add(u32::from(repeated));
    }
}
