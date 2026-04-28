//! Poll the pending voice transcription slot and inject text into input.

use crate::tui::app::state::AppState;

/// Check for a completed transcription and append text to the input buffer.
/// Called from the tick loop. Clears the slot after consuming.
pub fn drain_voice_transcription(state: &mut AppState) {
    let slot = match &state.pending_voice_text {
        Some(s) => s.clone(),
        None => return,
    };
    let text = match slot.lock().ok().and_then(|mut g| g.take()) {
        Some(t) if !t.is_empty() => t,
        _ => return,
    };
    state.pending_voice_text = None;
    state.insert_text(&text);
    state.status = "Voice transcribed ✓".into();
}
