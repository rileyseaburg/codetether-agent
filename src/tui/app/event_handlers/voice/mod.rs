//! Ctrl+R handler: toggle voice recording from the TUI.
//!
//! First press starts recording, second press stops and transcribes.
//! Auto-stops after `CODETETHER_VOICE_INPUT_MAX_SECS` (default 60).
//! The tick loop polls [`voice_drain::drain_voice_transcription`].

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::tool::voice_input::{encoder, recorder};
use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

use voice_transcribe::transcribe_sync;

mod voice_drain;
mod voice_transcribe;

/// Maximum recording duration from env (default 60s).
fn max_duration() -> u64 {
    std::env::var("CODETETHER_VOICE_INPUT_MAX_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(60)
        .min(300)
}

/// Handle Ctrl+R keypress — toggle voice recording.
pub(super) fn handle_voice_input(app: &mut App) {
    if app.state.view_mode != ViewMode::Chat {
        return;
    }
    match &app.state.recording_stop_flag {
        Some(flag) => {
            flag.store(true, Ordering::Relaxed);
            app.state.status = "Transcribing...".into();
            app.state.recording_stop_flag = None;
        }
        None => {
            let flag = Arc::new(AtomicBool::new(false));
            app.state.recording_stop_flag = Some(flag.clone());
            app.state.status = "Recording... (Ctrl+R to stop)".into();
            let slot = Arc::new(std::sync::Mutex::new(None));
            app.state.pending_voice_text = Some(slot.clone());
            let max = max_duration();
            std::thread::spawn(move || match recorder::record(max, flag) {
                Ok(samples) => {
                    if let Ok(wav) = encoder::encode_wav(&samples) {
                        if let Ok(mut guard) = slot.lock() {
                            *guard = transcribe_sync(&wav);
                        }
                    }
                }
                Err(e) => tracing::error!("Recording failed: {e}"),
            });
        }
    }
}

pub use voice_drain::drain_voice_transcription;
