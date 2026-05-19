//! Background voice recording and transcription worker.

use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use crate::tool::voice_input::{encoder, recorder};

use super::voice_transcribe::transcribe_sync;

type VoiceSlot = Arc<Mutex<Option<String>>>;

/// Start recording audio and publish the transcription result to `slot`.
pub(super) fn spawn_voice_worker(max: u64, flag: Arc<AtomicBool>, slot: VoiceSlot) {
    std::thread::spawn(move || publish_voice_text(&slot, record_text(max, flag)));
}

fn record_text(max: u64, flag: Arc<AtomicBool>) -> String {
    match recorder::record(max, flag) {
        Ok(samples) => encoder::encode_wav(&samples)
            .ok()
            .and_then(|wav| transcribe_sync(&wav))
            .unwrap_or_default(),
        Err(e) => {
            tracing::error!(error = %e, "Recording failed");
            String::new()
        }
    }
}

fn publish_voice_text(slot: &VoiceSlot, text: String) {
    if let Ok(mut guard) = slot.lock() {
        *guard = Some(text);
    }
}
