//! Paddle movement and board-state queries.
use crate::{InterludeState, LANES, ROWS, rules};
impl InterludeState {
    /// Move left without crossing the edge.
    pub fn left(&mut self) {
        self.lane = self.lane.saturating_sub(1);
    }
    /// Move right without crossing the edge.
    pub fn right(&mut self) {
        self.lane = (self.lane + 1).min(LANES - 1);
    }
    /// Return the current paddle lane.
    pub fn lane(&self) -> u8 {
        self.lane
    }
    /// Return the number of stars caught.
    pub fn score(&self) -> u32 {
        self.score
    }
    /// Return the current star lane and row.
    pub fn star(&self) -> (u8, u8) {
        (
            rules::star_lane(self.tick / u64::from(ROWS)),
            self.tick as u8 % ROWS,
        )
    }
}
