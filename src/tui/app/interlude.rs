//! State and rules for the Tether Catch play break.

use std::time::{Duration, Instant};

#[path = "interlude_rules.rs"]
mod rules;

pub(crate) const LANES: u8 = 5;
pub(crate) const ROWS: u8 = 7;

pub(crate) struct InterludeState {
    started: Instant,
    lane: u8,
    score: u32,
    tick: u64,
}

impl InterludeState {
    pub(crate) fn new() -> Self {
        Self {
            started: Instant::now(),
            lane: LANES / 2,
            score: 0,
            tick: 0,
        }
    }

    pub(crate) fn left(&mut self) {
        self.lane = self.lane.saturating_sub(1);
    }

    pub(crate) fn right(&mut self) {
        self.lane = (self.lane + 1).min(LANES - 1);
    }

    pub(crate) fn advance(&mut self) {
        self.advance_to(self.started.elapsed());
    }

    fn advance_to(&mut self, elapsed: Duration) {
        rules::advance(&mut self.tick, self.lane, &mut self.score, elapsed);
    }

    pub(crate) fn lane(&self) -> u8 {
        self.lane
    }
    pub(crate) fn score(&self) -> u32 {
        self.score
    }
    pub(crate) fn star(&self) -> (u8, u8) {
        (
            rules::star_lane(self.tick / u64::from(ROWS)),
            self.tick as u8 % ROWS,
        )
    }
}

#[cfg(test)]
#[path = "interlude_tests.rs"]
mod tests;
