//! Time-driven scoring for the play break.
use crate::{InterludeState, rules};
use std::time::Duration;
impl InterludeState {
    /// Advance scoring to the elapsed wall-clock time.
    pub fn advance(&mut self) {
        self.advance_to(self.started.elapsed());
    }
    /// Advance scoring to an explicit elapsed duration.
    pub(super) fn advance_to(&mut self, elapsed: Duration) {
        rules::advance(&mut self.tick, self.lane, &mut self.score, elapsed);
    }
}
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
