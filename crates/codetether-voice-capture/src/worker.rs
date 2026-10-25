//! Background voice recording and transcription worker.

use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use codetether_voice_input_tool::{encoder, recorder};

use super::{level, transcribe::transcribe_sync};

/// Outcome of one recording: transcribed text, or a user-facing failure reason.
///
/// # Examples
/// ```
/// let ok: codetether_voice_capture::VoiceOutcome = Ok("hello".into());
/// let failed: codetether_voice_capture::VoiceOutcome = Err("No audio captured".into());
/// assert!(ok.is_ok() && failed.is_err());
/// ```
pub type VoiceOutcome = Result<String, String>;

/// Caller-owned slot that receives the completed [VoiceOutcome].
/// # Examples
/// ```
/// let slot: codetether_voice_capture::VoiceSlot = Default::default();
/// assert!(slot.lock().unwrap().is_none());
/// ```
pub type VoiceSlot = Arc<Mutex<Option<VoiceOutcome>>>;

/// Start recording audio and publish the transcription outcome to `slot`.
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

fn record_text(max: u64, flag: Arc<AtomicBool>) -> VoiceOutcome {
    let samples = recorder::record(max, flag).map_err(|e| {
        tracing::error!(target: "codetether_agent::tui::app::event_handlers::voice::voice_worker", error = %e, "Recording failed");
        format!("Recording failed: {e}. {}", level::MIC_HINT)
    })?;
    let peak = level::peak(&samples);
    tracing::info!(target: "codetether_agent::tui::app::event_handlers::voice::voice_worker", samples = samples.len(), peak, "Recording captured");
    level::check(&samples)?;
    let wav = encoder::encode_wav(&samples).map_err(|e| format!("WAV encoding failed: {e}"))?;
    transcribe_sync(&wav)
}

fn publish_voice_text(slot: &VoiceSlot, outcome: VoiceOutcome) {
    if let Ok(mut guard) = slot.lock() {
        *guard = Some(outcome);
    }
}
