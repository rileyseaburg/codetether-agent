use std::time::{Duration, Instant};
const STREAM_FRAME_INTERVAL: Duration = Duration::from_millis(66);
/// Record successful rendering; skipped frames retain dirty state and timing.
/// # Arguments
/// Needs_redraw and last_draw are updated only when rendered is true.
/// # Returns
/// No value; the caller's state records a completed draw if one occurred.
/// # Examples
/// ```
/// let mut dirty = true;
/// let mut last = None;
/// codetether_tui_tick::redraw::finish(&mut dirty, &mut last, false);
/// assert!(dirty && last.is_none());
/// ```
pub fn finish(needs_redraw: &mut bool, last_draw: &mut Option<Instant>, rendered: bool) {
    if rendered {
        *needs_redraw = false;
        *last_draw = Some(Instant::now());
    }
}

pub fn ready(processing: bool, last_input: Option<Instant>, last_draw: Option<Instant>) -> bool {
    let Some(last_draw) = last_draw else {
        return true;
    };
    if !processing || input_since(last_input, last_draw) {
        return true;
    }
    last_draw.elapsed() >= STREAM_FRAME_INTERVAL
}

fn input_since(last_input: Option<Instant>, last_draw: Instant) -> bool {
    last_input.is_some_and(|last_input| last_input >= last_draw)
}
