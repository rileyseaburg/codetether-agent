//! Poll the pending voice transcription slot and inject text into input.

use crate::tui::app::state::AppState;

/// Check for a completed transcription and append text to the input buffer.
/// Called from the tick loop. Clears the slot after consuming.
pub fn drain_voice_transcription(state: &mut AppState) {
    let slot = match &state.pending_voice_text {
        Some(s) => s.clone(),
        None => return,
    };
    let text = slot.lock().ok().and_then(|mut g| g.take());
    match text {
        Some(t) if !t.is_empty() => {
            state.pending_voice_text = None;
            state.insert_text(&t);
            state.status = "Voice transcribed ✓".into();
        }
        Some(_) => {
            state.pending_voice_text = None;
            state.status = "Voice transcription failed".into();
        }
        None => {}
    }
}
