//! Recording cancellation and pending transcription state.
use crate::VoiceSlot;
use std::sync::{Arc, atomic::AtomicBool};

/// State shared between interactive recording controls and result polling.
///
/// Creating this state does not start audio capture. The recording controls
/// install the cancellation flag and the worker's transcription slot.
///
/// # Examples
/// ```
/// let recording = codetether_voice_capture::RecordingState::default();
/// assert!(recording.stop_flag.is_none());
/// assert!(recording.pending_text.is_none());
/// ```
#[derive(Default)]
pub struct RecordingState {
    /// Stop flag for the current recording, if one has started.
    pub stop_flag: Option<Arc<AtomicBool>>,
    /// Worker result retained until a completed transcription is consumed.
    pub pending_text: Option<VoiceSlot>,
}
