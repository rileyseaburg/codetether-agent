//! Older blocking transcription drain retained separately from active polling.

use crate::Input;

/// Consume a nonempty result using the older blocking-lock behavior.
/// # Arguments
/// * `state` — Frontend slot, text insertion and status operations.
/// # Returns
/// Retains the pending handle when no nonempty text is available. Successful
/// results clear the handle and are inserted through the frontend.
/// # Examples
/// Implement [Input] to adapt a frontend that explicitly needs this behavior.
pub fn drain_voice_transcription(state: &mut impl Input) {
    let slot = match state.pending() {
        Some(s) => s.clone(),
        None => return,
    };
    let text = match slot.lock().ok().and_then(|mut g| g.take()) {
        Some(t) if !t.is_empty() => t,
        _ => return,
    };
    state.clear_pending();
    state.insert_text(&text);
    state.set_status("Voice transcribed ✓");
}
