//! Bind shared nonblocking transcription polling to the frontend input state.
use crate::tui::app::state::AppState;
use codetether_voice_capture::{Input, VoiceSlot};
impl Input for AppState {
    fn pending(&self) -> Option<&VoiceSlot> {
        self.pending_voice_text.as_ref()
    }
    fn clear_pending(&mut self) {
        self.pending_voice_text = None;
    }
    fn insert_text(&mut self, text: &str) {
        AppState::insert_text(self, text);
    }
    fn set_status(&mut self, status: &str) {
        self.status = status.into();
    }
}
/// Consume the pending result through the original nonblocking input path.
pub fn drain_voice_transcription(state: &mut AppState) {
    codetether_voice_capture::drain_voice_transcription(state);
}
