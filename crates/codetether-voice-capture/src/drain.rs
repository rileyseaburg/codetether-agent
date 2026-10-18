//! Poll the pending voice transcription slot and inject text into input.

use crate::Input;

/// Check for a completed transcription and append text to the input buffer.
/// Called from the tick loop. Clears the slot after consuming.
/// # Arguments
/// * `state` — Frontend slot, text insertion and status operations.
/// # Returns
/// Returns immediately when the slot is locked or incomplete; consumes completed
/// empty results and reports their status without inserting text.
/// # Examples
/// Frontends implement [Input] and call this function on each polling tick.
pub fn drain_voice_transcription(state: &mut impl Input) {
    let slot = match state.pending() {
        Some(s) => s.clone(),
        None => return,
    };
    let text = match slot.try_lock().ok().and_then(|mut g| g.take()) {
        Some(t) => t,
        None => return,
    };
    state.clear_pending();
    if text.is_empty() {
        state.set_status("Voice transcription produced no text");
        return;
    }
    state.insert_text(&text);
    state.set_status("Voice transcribed ✓");
}
