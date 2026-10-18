//! Background voice recording and transcription worker.

use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use codetether_voice_input_tool::{encoder, recorder};

use super::transcribe::transcribe_sync;

/// Caller-owned slot that receives a completed transcription or an empty string.
/// # Examples
/// ```
/// let slot: codetether_voice_capture::VoiceSlot = Default::default();
/// assert!(slot.lock().unwrap().is_none());
/// ```
pub type VoiceSlot = Arc<Mutex<Option<String>>>;

/// Start recording audio and publish the transcription result to `slot`.
/// # Arguments
/// * `max` — Maximum recording duration in seconds.
/// * `flag` — Caller-controlled early-stop flag.
/// * `slot` — Result slot updated after recording and transcription complete.
/// # Returns
/// Starts the detached worker thread and returns immediately.
/// # Panics
/// Panics if the operating system refuses to create the thread.
/// # Examples
/// ```no_run
/// use std::sync::{Arc, atomic::AtomicBool};
/// let slot = Default::default();
/// codetether_voice_capture::spawn_voice_worker(60, Arc::new(AtomicBool::new(false)), slot);
/// ```
pub fn spawn_voice_worker(max: u64, flag: Arc<AtomicBool>, slot: VoiceSlot) {
    std::thread::spawn(move || publish_voice_text(&slot, record_text(max, flag)));
}

fn record_text(max: u64, flag: Arc<AtomicBool>) -> String {
    match recorder::record(max, flag) {
        Ok(samples) => encoder::encode_wav(&samples)
            .ok()
            .and_then(|wav| transcribe_sync(&wav))
            .unwrap_or_default(),
        Err(e) => {
            tracing::error!(target: "codetether_agent::tui::app::event_handlers::voice::voice_worker", error = %e, "Recording failed");
            String::new()
        }
    }
}

fn publish_voice_text(slot: &VoiceSlot, text: String) {
    if let Ok(mut guard) = slot.lock() {
        *guard = Some(text);
    }
}
