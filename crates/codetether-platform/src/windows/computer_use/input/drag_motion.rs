//! Stepped cursor movement for drag gestures.

use std::time::Duration;

use super::move_cursor;

const MAX_STEPS: u32 = 240;
const MAX_DURATION_MS: u64 = 30_000;

pub fn drag_motion(
    start: (i32, i32),
    end: (i32, i32),
    steps: Option<u32>,
    duration_ms: Option<u64>,
) -> anyhow::Result<()> {
    let steps = steps.unwrap_or(1);
    anyhow::ensure!(
        steps > 0 && steps <= MAX_STEPS,
        "steps must be 1..={MAX_STEPS}"
    );
    let duration_ms = duration_ms.unwrap_or(100);
    anyhow::ensure!(
        duration_ms <= MAX_DURATION_MS,
        "duration_ms must be <= {MAX_DURATION_MS}"
    );
    let delay = Duration::from_millis(duration_ms / steps as u64);
    for step in 1..=steps {
        move_cursor(
            interpolate(start.0, end.0, step, steps),
            interpolate(start.1, end.1, step, steps),
        )?;
        if !delay.is_zero() {
            std::thread::sleep(delay);
        }
    }
    Ok(())
}

fn interpolate(start: i32, end: i32, step: u32, steps: u32) -> i32 {
    let delta = i64::from(end) - i64::from(start);
    (i64::from(start) + delta * i64::from(step) / i64::from(steps)) as i32
}
