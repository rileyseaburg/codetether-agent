//! Owned clock and board position for one play break.
use crate::LANES;
use std::time::Instant;
/// Paddle position, score, and elapsed animation steps.
///
/// # Examples
/// ```
/// let game = codetether_interlude::InterludeState::new();
/// assert_eq!(game.score(), 0);
/// ```

pub struct InterludeState {
    pub(super) started: Instant,
    pub(super) lane: u8,
    pub(super) score: u32,
    pub(super) tick: u64,
}
impl InterludeState {
    /// Create a centered paddle with zero score and a fresh clock.
    pub fn new() -> Self {
        Self {
            started: Instant::now(),
            lane: LANES / 2,
            score: 0,
            tick: 0,
        }
    }
}
impl Default for InterludeState {
    fn default() -> Self {
        Self::new()
    }
}
