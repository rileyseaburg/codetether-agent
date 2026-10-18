//! Borrowed recording state with frontend-selected text insertion.
use crate::{Input, RecordingState, VoiceSlot};

/// Supplies pending transcription and status fields to the polling workflow.
///
/// The insertion callback retains the frontend's normal composer behavior.
///
/// # Examples
/// ```
/// use codetether_voice_capture::{Input, RecordingState, TranscriptionInput};
/// let mut recording = RecordingState::default();
/// let mut status = String::new();
/// let mut text = String::new();
/// let mut input = TranscriptionInput {
///     recording: &mut recording, status: &mut status,
///     insert: |value: &str| text.push_str(value),
/// };
/// assert!(input.pending().is_none());
/// input.insert_text("hello");
/// drop(input);
/// assert_eq!(text, "hello");
/// ```
pub struct TranscriptionInput<'a, F> {
    /// Recording lifecycle and pending result.
    pub recording: &'a mut RecordingState,
    /// Frontend status output.
    pub status: &'a mut String,
    /// Insert text through the frontend's normal input path.
    pub insert: F,
}

impl<F: FnMut(&str)> Input for TranscriptionInput<'_, F> {
    fn pending(&self) -> Option<&VoiceSlot> {
        self.recording.pending_text.as_ref()
    }
    fn clear_pending(&mut self) {
        self.recording.pending_text = None;
    }
    fn insert_text(&mut self, text: &str) {
        (self.insert)(text);
    }
    fn set_status(&mut self, status: &str) {
        *self.status = status.into();
    }
}
