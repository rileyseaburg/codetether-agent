//! Bind transcription polling to the existing composer insertion workflow.
use crate::tui::app::state::AppState;

/// Consume the pending result through the original nonblocking input path.
pub fn drain_voice_transcription(state: &mut AppState) {
    let mut composer = codetether_composer_view::interactive::Composer {
        input: &mut state.input,
        cursor: &mut state.input_cursor,
        scroll: &mut state.input_scroll,
        mode: &mut state.input_mode,
        completion: &mut state.completion,
        history: &mut state.history,
    };
    let mut input = codetether_voice_capture::TranscriptionInput {
        recording: &mut state.recording,
        status: &mut state.status,
        insert: |text: &str| composer.insert_text(text),
    };
    codetether_voice_capture::drain_voice_transcription(&mut input);
}
