use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use crate::tool::voice_input::{encoder, recorder};

use super::voice_transcribe::transcribe_sync;

pub(super) fn spawn(slot: Arc<Mutex<Option<String>>>, max_duration: u64, flag: Arc<AtomicBool>) {
    std::thread::spawn(move || finish(slot, capture_text(max_duration, flag)));
}

fn capture_text(max_duration: u64, flag: Arc<AtomicBool>) -> String {
    let wav = match recorder::record(max_duration, flag).and_then(|samples| {
        if samples.is_empty() {
            anyhow::bail!("No audio captured. Check your microphone.");
        }
        encoder::encode_wav(&samples)
    }) {
        Ok(wav) => wav,
        Err(e) => {
            tracing::error!("Voice capture failed: {e}");
            return String::new();
        }
    };
    transcribe_sync(&wav).unwrap_or_default()
}

fn finish(slot: Arc<Mutex<Option<String>>>, text: String) {
    if let Ok(mut guard) = slot.lock() {
        *guard = Some(text);
    }
}
