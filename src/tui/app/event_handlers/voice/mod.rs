//! Ctrl+R voice controls bound to existing frontend state.
#![allow(dead_code)]
use crate::tui::{app::state::App, models::ViewMode};
mod voice_drain;
/// Handle the recording toggle with the shared voice lifecycle.
pub(super) fn handle_voice_input(app: &mut App) {
    codetether_voice_capture::toggle_recording(codetether_voice_capture::Controls {
        is_chat: app.state.view_mode == ViewMode::Chat,
        recording_stop_flag: &mut app.state.recording.stop_flag,
        pending_voice_text: &mut app.state.recording.pending_text,
        status: &mut app.state.status,
    });
}
pub use voice_drain::drain_voice_transcription;
