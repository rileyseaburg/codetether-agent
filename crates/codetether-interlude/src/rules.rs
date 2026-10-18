//! Time-step and scoring rules for Tether Catch.

use std::time::Duration;

use super::{LANES, ROWS};

const STEP_MILLIS: u128 = 250;

pub(super) fn advance(tick: &mut u64, lane: u8, score: &mut u32, elapsed: Duration) {
    let target = (elapsed.as_millis() / STEP_MILLIS) as u64;
    while *tick < target {
        *tick += 1;
        if *tick % u64::from(ROWS) == u64::from(ROWS - 1)
            && lane == star_lane(*tick / u64::from(ROWS))
        {
            *score += 1;
        }
    }
}

pub(super) fn star_lane(cycle: u64) -> u8 {
    ((cycle * 3 + 1) % u64::from(LANES)) as u8
}
