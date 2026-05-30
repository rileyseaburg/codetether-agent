//! Ctrl+R handler: toggle voice recording from the TUI.

#![allow(dead_code)]

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

use voice_worker::spawn_voice_worker;

mod voice_drain;
mod voice_transcribe;
mod voice_worker;

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
            if app.state.pending_voice_text.is_some() {
                app.state.status = "Still transcribing previous recording...".into();
                return;
            }
            let flag = Arc::new(AtomicBool::new(false));
            app.state.recording_stop_flag = Some(flag.clone());
            app.state.status = "Recording... (Ctrl+R to stop)".into();
            let slot = Arc::new(std::sync::Mutex::new(None));
            app.state.pending_voice_text = Some(slot.clone());
            spawn_voice_worker(max_duration(), flag, slot);
        }
    }
}

pub use voice_drain::drain_voice_transcription;
