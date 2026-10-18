//! Recording-state transitions for interactive voice input.
use crate::{VoiceSlot, max_duration, spawn_voice_worker};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
/// Borrowed frontend fields used by the recording toggle.
/// # Examples
/// ```
/// use codetether_voice_capture::{Controls, toggle_recording};
/// let (mut stop, mut pending, mut status) = (None, None, String::new());
/// toggle_recording(Controls { is_chat: false, recording_stop_flag: &mut stop, pending_voice_text: &mut pending, status: &mut status });
/// assert!(stop.is_none());
/// ```
pub struct Controls<'a> {
    /// Whether the frontend is showing the chat view.
    pub is_chat: bool,
    /// Current recording's stop flag, cleared when stopping.
    pub recording_stop_flag: &'a mut Option<Arc<AtomicBool>>,
    /// Pending worker result, retained until transcription is consumed.
    pub pending_voice_text: &'a mut Option<VoiceSlot>,
    /// Status text displayed by the frontend.
    pub status: &'a mut String,
}
/// Start or stop recording using the existing frontend transition rules.
/// # Arguments
/// * `state` — Chat eligibility, recording flag, pending result and status.
/// # Returns
/// Updates borrowed state and starts the worker only for an eligible new recording.
/// # Panics
/// Retains worker-thread creation failure behavior when starting recording.
/// # Examples
/// See [Controls] for a non-recording example.
pub fn toggle_recording(state: Controls<'_>) {
    if !state.is_chat {
        return;
    }
    match &*state.recording_stop_flag {
        Some(flag) => {
            flag.store(true, Ordering::Relaxed);
            *state.status = "Transcribing...".into();
            *state.recording_stop_flag = None;
        }
        None => {
            if state.pending_voice_text.is_some() {
                *state.status = "Still transcribing previous recording...".into();
                return;
            }
            let flag = Arc::new(AtomicBool::new(false));
            *state.recording_stop_flag = Some(flag.clone());
            *state.status = "Recording... (Ctrl+R to stop)".into();
            let slot = Arc::new(std::sync::Mutex::new(None));
            *state.pending_voice_text = Some(slot.clone());
            spawn_voice_worker(max_duration(), flag, slot);
        }
    }
}
